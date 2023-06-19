use youtube_rs::*;

fn main() {
    let client = YTClient::from_secret_path("src/secret.json").unwrap();
    let rating = data_structs::LikingArgs {
        id:"sg4TxfwSeYs",
        rating:VideoInteration::Like
    };
    client.rate_video_request(rating).unwrap();
}
