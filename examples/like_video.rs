use youtube_rs::liking::*;
use youtube_rs::YTClient;

fn main() {
    let client = YTClient::from_secret_path("src/secret.json").unwrap();
    let rating = LikingArgs {
        id:"sg4TxfwSeYs",
        rating:VideoInteration::Like
    };
    client.rate_video_request(rating).unwrap();
}
