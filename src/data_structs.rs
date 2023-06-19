#[path ="options.rs"]
mod options;
pub use crate::data_structs::options::*;
pub use crate::liking::*;

#[derive(Debug,Clone)]
pub enum PrivacyStatus {
    Public,
    Private,
    Unlisted
}

impl PrivacyStatus {
    pub fn to_str(&self) -> &str{
        match self {
            PrivacyStatus::Private => "private",
            PrivacyStatus::Public => "public",
            PrivacyStatus::Unlisted => "unlisted"
        }
    }
}
#[derive(Debug,Clone)]
pub struct VideoData<'a,'b,'c,'d> {
    pub title:&'a str,
    pub desc:&'b str,
    pub keywords:Option<&'c str>,
    pub category:u32,
    pub privacy_status:PrivacyStatus,
    pub file:&'d str,
    pub for_kids:bool

}

#[derive(Debug,Clone)]
pub struct ThumbnailArgs<'a,'b> {
    pub id:&'a str,
    pub file:&'b str,
}

#[repr(u32)]
/// A couple Youtube CategoryIDs. To see more CategoryIDs visit (https://mixedanalytics.com/blog/list-of-youtube-video-category-ids/).
pub enum CategoryID {
    FilmAnimation = 1,
    AutosVehicles = 2,
    Music = 10,
    PetAnimals = 15,
    Sports = 17,
    ShortMovies = 18,
    TravelEvents = 19,
    Gaming = 20,
    Blogging = 21,
    Comedy = 23,
    Entertainment = 24,
    PeopleBlogs = 22,
    NewsPolitics = 25,
    Education = 27,
    SciTech = 28,
    Movies = 30,
    Animation = 31,
    ActionAdventure = 32,
    Documentary = 35,
    Shorts = 42
}
