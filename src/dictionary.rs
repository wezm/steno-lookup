use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
struct Stroke(String);

#[derive(Debug, Deserialize)]
struct Translation(String);

#[derive(Debug, Deserialize)]
pub struct Dictionary(HashMap<Stroke, Translation>);

impl Dictionary {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path)?;

        serde_json::from_reader(file).map_err(Error::from)
    }
}
