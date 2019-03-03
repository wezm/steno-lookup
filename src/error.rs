use std::io;

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Io(io::Error),
    Ini(ini::ini::Error),
    SectionMissing,
    ConfigNotFound,
    HomeNotFound,
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

impl From<ini::ini::Error> for Error {
    fn from(error: ini::ini::Error) -> Self {
        Error::Ini(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}
