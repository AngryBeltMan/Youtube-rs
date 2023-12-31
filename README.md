# Youtube-rs
# About
Youtube-rs is a simple easy to use youtube api library make for the programming language rust.
The library is still a work and progress so expect new features to be out or unexpected errors.
The library uses python binding to call the youtube api. This library currently supports uploading
videos, editing thumbnails, and rating videos.
# Getting Started
First install pip if you havent already. Next install all of the required modules with pip3 by pasting the following command into a terminal.
```
pip3 install google-api-python-client==1.7.2 && 
pip3 install google-auth==1.8.0 &&
pip3 install google-auth-httplib2==0.0.3 &&
pip3 install google-auth-oauthlib==0.4.1
```
# Tutorial
Watch this simple video to find out how to upload a video https://www.youtube.com/watch?v=ABWl6vFP2uU

# Example
Here is how to upload a video using this library.
```
use youtube_rs::*;
fn main() {
    // Put your own youtube secret file here
    let client = YTClient::from_secret_path("./secret.json").unwrap();
    let options = VideoData {
        title: "test video",
        desc: "cool description",
        keywords: Some("test,cool"),
        category:video::CategoryID::SciTech as u32,
        privacy_status: video::PrivacyStatus::Private,
        file: "./test.mp4",
        for_kids:false
    };
    // Creates the settings for the video
    let opt = client.create_upload_options(options).unwrap();
    client.upload_request(opt).expect("Could not upload");
}
```
