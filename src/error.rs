use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Io(io::Error),
    Ini(ini::ini::ParseError),
    SectionMissing,
    FileNotFound(PathBuf),
    HomeNotFound,
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

impl From<ini::ini::ParseError> for Error {
    fn from(error: ini::ini::ParseError) -> Self {
        Error::Ini(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}
