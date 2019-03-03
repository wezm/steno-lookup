use std::io;
use std::path::Path;

use ini::Ini;
use serde::Deserialize;

use crate::Error;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct DictionaryConfig {
    pub enabled: bool,
    pub path: String,
}

/// Extract the dictionary list from the supplied plover config path
///
/// `section_name` is the section in the config file to read for the dictionaries. Such as
/// "System: English Stenotype".
pub fn dictionaries<P: AsRef<Path>>(
    path: P,
    section_name: &str,
) -> Result<Vec<DictionaryConfig>, Error> {
    let cfg = Ini::load_from_file(&path).map_err(|err| match err {
        ini::ini::Error::Io(io_err) => match io_err.kind() {
            io::ErrorKind::NotFound => Error::FileNotFound(path.as_ref().to_path_buf()),
            _ => Error::from(io_err),
        },
        ini::ini::Error::Parse(err) => Error::from(err),
    })?;

    let dictionaries = cfg
        .get_from(Some(section_name), "dictionaries")
        .ok_or_else(|| Error::SectionMissing)?;
    serde_json::from_str(dictionaries).map_err(Error::from)
}
