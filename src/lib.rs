#![warn(clippy::all)]
use cpython::*;
use crate::thumbnail::ThumbnailArgs;
use crate::liking::LikingArgs;
pub use crate::video::*;
use std::{fs::File, io::Read, path::Path};
/// Video Rating
/// ```
/// use youtube_rs::liking::*;
/// use youtube_rs::YTClient;
///
/// fn main() {
///     let client = YTClient::from_secret_path("src/secret.json").unwrap();
///     let rating = LikingArgs {
///         // ID of the video you are going to like
///         id:"sg4TxfwSeYs",
///         // Type of rating you are going to give
///         rating:VideoInteration::Like
///     };
///     client.rate_video_request(rating).unwrap();
/// }
/// ```
pub mod liking;
/// Thumbnail Editing
pub mod thumbnail;
/// Video Uploading
/// ```
/// // Put your own youtube secret file here
/// let client = YTClient::from_secret_path("./secret.json").unwrap();
/// let options = VideoData {
///     title: "test video",
///     desc: "cool description",
///     keywords: Some("test,cool"),
///     category:video::CategoryID::SciTech as u32,
///     privacy_status: video::PrivacyStatus::Private,
///     file: "./test.mp4",
///     for_kids:false
/// };
/// Creates the settings for the video
/// let opt = client.create_upload_options(options).unwrap();
/// client.upload_request(opt).expect("Could not upload");
/// ```
pub mod video;

static PYLIB:&str = include_str!("../pythonlib/lib.py");

/// The client that is used to interface with the google youtube api.
/// ```
/// /* When inputing the secret make sure it is the oauth2 token and make sure it is a desktop
/// application type */
/// YTClient::new_from_path("./secret.json")
/// ```
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
    /// DEPRICATED. This method is deprecated because it required the lib files to be present in
    /// user rust program. Call new_from_secret(...) to create a new youtube client from a str.
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
    /// Returns a struct that can be used to upload a video. Will return an error is the file given
    /// doesn't exist.
    pub fn create_upload_options(&self,video_data:VideoData) -> Result<UploadOptions,String> {
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
    pub fn rate_video_request(&self,rating_data:LikingArgs) -> Result<(),impl std::fmt::Debug> {
        let py = self.gil.python();
        let interaction = self.module.call_method(py, "InteractionArgs", (rating_data.id,rating_data.rating.to_str()), None)?;
        self.module.call_method(py, "interact_with_video", (self.client.clone_ref(py),interaction), None)?;
        Ok::<(),PyErr>(())
    }
    /// Sends a request to upload a new video.
    pub fn upload_request(&self,opt:UploadOptions) -> Result<(),impl std::fmt::Debug> {
        let py = self.gil.python();
        self.module.call_method(py, "upload_req", (self.client.clone_ref(py),opt.options), None)?;
        Ok::<(),PyErr>(())
    }
    /// Sends a request to change the thumbnail of one of your videos.
    pub fn set_thumbnail(&self,args:ThumbnailArgs) -> Result<(),impl std::fmt::Debug>{
        let py = self.gil.python();
        self.module.call_method(py, "set_thumbnail", (args.id,args.file), None)?;
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
        category:video::CategoryID::SciTech as u32,
        privacy_status:video::PrivacyStatus::Private,
        file:"src/funny.mp4",
        for_kids:false
    };
    let opt = client.create_upload_options(video_data).unwrap();
    client.upload_request(opt).unwrap();
}
