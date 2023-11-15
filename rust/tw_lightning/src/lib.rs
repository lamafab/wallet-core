pub mod entry;
pub mod modules;
pub mod mapping;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
// TODO:
pub struct Error();
