/// Provides arguments for creating a new thumbnail
#[derive(Debug,Clone)]
pub struct ThumbnailArgs<'a,'b> {
    pub id:&'a str,
    pub file:&'b str,
}
