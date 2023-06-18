use cpython::*;
use std::{fs::File, io::Read, path::Path};
use crate::data_structs::Options;
mod data_structs;

pub struct YTClient {
    gil:GILGuard,
    module:PyObject,
    client:PyObject
}
/// The YTClient is required for calling and using the youtube api in this library.
impl YTClient {
    /// Creates the client using thhe google youtube api secret json file. You can get the secret
    /// json file from the google developers page. If you want to create the client using the path
    /// the secret json file use the from_path(...) method instead.
    pub fn from_str(data:&str) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();
        // This will be used to import the lib module using import lib
        // Okay to unwrap because they are all valid functions
        let importlib = py.import("importlib.util").unwrap();
        let sys = py.import("sys").unwrap();
        // location of the library
        let spec = importlib.call(py, "spec_from_file_location", ("lib","./pythonlib/lib.py",), None).unwrap();
        let module = importlib.call(py, "module_from_spec", (spec.clone_ref(py),), None).unwrap();
        let modules = sys.get(py, "modules").unwrap();
        modules.set_item(py, "lib", module.clone_ref(py)).unwrap();
        let loader = spec.getattr(py, "loader").unwrap();
        loader.call_method(py, "exec_module", (module.clone_ref(py),), None).unwrap();

        // Begin to create the client
        let client = module.call_method(py, "client_from_str", (data,), None).unwrap();
        Self { gil,module,client }
    }
    /// Creates the client using the path to the secret json file as input.
    pub fn from_path(path:&str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let client = Self::from_str(&buf);
        Ok(client)
    }
    // self,title:str,desc:str,keywords:str,category:str,privacy_status:str,file:str,for_kids=False
    pub fn create_options(&self,title:&str,desc:&str,keywords:Option<&str>,category:u32,privacy_status:data_structs::PrivacyStatus,file:&str,for_kids:bool) -> Result<Options,String> {
        let py = self.gil.python();
        if !Path::new(file).exists() { return Err(String::from("File does not exist")) }
        let opt = self.module.call_method(py, "Options", (title,desc,keywords.unwrap_or(""),&category.to_string(),privacy_status.to_str(),file,for_kids), None).unwrap();
        Ok(Options { options: opt })
    }
    pub fn upload_request(&self,opt:Options) -> Result<(),impl std::fmt::Debug> {
        let py = self.gil.python();
        self.module.call_method(py, "upload_req", (self.client.clone_ref(py),opt.options), None)?;
        Ok::<(),PyErr>(())
    }
}


#[test]
fn test_client() {
    let client = YTClient::from_path("src/secret.json").unwrap();
    let opt = client.create_options("cool", "bruh", None, 22, data_structs::PrivacyStatus::Public, "src/test.mp4", false).unwrap();
    client.upload_request(opt).unwrap();
}
