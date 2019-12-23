#[derive(Debug, Clone)]
pub enum Error {
    InsufficientStream,
}

pub type Result<T> = std::result::Result<T, Error>;
