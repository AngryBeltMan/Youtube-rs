use cpython::PyObject;

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
