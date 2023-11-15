pub mod entry;
pub mod mapping;
pub mod modules;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
// TODO:
pub struct Error();
