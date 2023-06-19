use cpython::*;
use data_structs::VideoData;
use std::{fs::File, io::{Read,Write}, path::Path};
pub use crate::data_structs::UploadOptions;
pub mod data_structs;
static PYLIB:&str = include_str!("../pythonlib/lib.py");
static PYSEARCH:&str = include_str!("../pythonlib/search.py");

pub struct YTClient {
    gil:GILGuard,
    module:PyObject,
    client:PyObject
}
/// The YTClient is required for calling and using the youtube api in this library.
impl YTClient {
    pub fn new_from_secret(data:&str) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();
        // Included in the python standard library
        let types = py.import("types").unwrap();
        let module = types.call(py, "ModuleType",("lib",),None).unwrap();
        let local = PyDict::new(py);
        local.set_item(py, "code", PYLIB).unwrap();
        local.set_item(py, "m", module.clone_ref(py)).unwrap();
        py.run("exec(code,m.__dict__)", Some(&local), None).unwrap();
        let client = module.call_method(py, "client_from_str", (data,), None).unwrap();
        Self { gil, module, client }
    }
    /// Creates the client using thhe google youtube api secret json file. You can get the secret
    /// json file from the google developers page. If you want to create the client using the path
    /// the secret json file use the from_path(...) method instead.
    #[deprecated]
    pub fn from_secret(data:&str) -> Self {
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
    pub fn from_secret_path(path:&str) -> std::io::Result<Self> {
        let dir = std::env::current_dir().unwrap();
        println!("{dir:?}");
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let client = Self::new_from_secret(&buf);
        Ok(client)
    }
    // self,title:str,desc:str,keywords:str,category:str,privacy_status:str,file:str,for_kids=False
    pub fn create_options(&self,video_data:VideoData) -> Result<UploadOptions,String> {
        let py = self.gil.python();
        if !Path::new(video_data.file).exists() { return Err(String::from("File does not exist")) }
        let opt = self.module.call_method( py, "Options", (
                video_data.title,
                 video_data.desc,
                 video_data.keywords.unwrap_or(""),
                 &video_data.category.to_string(),
                 video_data.privacy_status.to_str(),
                 video_data.file,
                 video_data.for_kids
             ), None).unwrap();
        Ok(UploadOptions { options: opt })
    }
    pub fn upload_request(&self,opt:UploadOptions) -> Result<(),impl std::fmt::Debug> {
        let py = self.gil.python();
        self.module.call_method(py, "upload_req", (self.client.clone_ref(py),opt.options), None)?;
        Ok::<(),PyErr>(())
    }
}


#[test]
fn test_client() {
    let client = YTClient::from_secret_path("src/secret.json").unwrap();
    let video_data = VideoData {
        title: "cool",
        desc: "cool video",
        keywords: Some("cool,video"),
        category:data_structs::CategoryID::SciTech as u32,
        privacy_status:data_structs::PrivacyStatus::Private,
        file:"src/funny.mp4",
        for_kids:false
    };
    let opt = client.create_options(video_data).unwrap();
    client.upload_request(opt).unwrap();
}
// #[test]
// fn new_test_client() {
//     YTClient::new_from_secret("hello");
// }
