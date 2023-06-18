use youtube_rs::*;
fn main() {
    // Put your own youtube secret file here
    let client = YTClient::from_path("./secret.json").unwrap();
    // Creates the settings for the video
    let opt = client.create_options("test video", "cool desciption", Some("test,cool"), data_structs::CategoryID::SciTech as u32, data_structs::PrivacyStatus::Private, "./test.mp4", false).unwrap();
    client.upload_request(opt).expect("Could not upload");
}
