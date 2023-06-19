use youtube_rs::{*, data_structs::VideoData};
fn main() {
    // Put your own youtube secret file here
    let client = YTClient::from_secret_path("./secret.json").unwrap();
    let options = VideoData {
        title: "test video",
        desc: "cool description",
        keywords: Some("test,cool"),
        category:data_structs::CategoryID::SciTech as u32,
        privacy_status: data_structs::PrivacyStatus::Private,
        file: "./test.mp4",
        for_kids:false
    };
    // Creates the settings for the video
    let opt = client.create_options(options).unwrap();
    client.upload_request(opt).expect("Could not upload");
}
