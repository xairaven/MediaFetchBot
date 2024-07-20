#[derive(PartialEq, Eq, Hash)]
pub enum RawMedia {
    Image(String),
    Text,
    Video(String)
}