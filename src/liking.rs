pub enum VideoInteration {
    Like,
    Dislike,
    Neither,
}
impl VideoInteration {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Like => "like",
            Self::Dislike => "dislike",
            Self::Neither => "none",
        }
    }
}
pub struct LikingArgs<'a> {
    pub id: &'a str,
    pub rating: VideoInteration,
}
