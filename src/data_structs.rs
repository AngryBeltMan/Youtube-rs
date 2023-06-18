use cpython::PyObject;

/// Setting for the video being uploaded.In order to be created a YTClient must be created first.
pub struct Options {
    pub options:PyObject
}
pub enum PrivacyStatus {
    Public,
    Private,
    Unlisted
}
impl PrivacyStatus {
    pub fn to_str(&self) -> &str{
        match self {
            PrivacyStatus::Private => return "private",
            PrivacyStatus::Public => return "public",
            PrivacyStatus::Unlisted => return "unlisted"
        }
    }
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
