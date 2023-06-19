use cpython::PyObject;
/// Setting for the video being uploaded.In order to be created a YTClient must be created first.
pub struct UploadOptions {
    pub options:PyObject
}
/// Options for interating with a video (ie liking and disliking).
pub struct InterationOptions {
    pub options:PyObject
}
