use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Stroke(String);

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Translation(String);

#[derive(Debug, Deserialize)]
pub struct Dictionary(HashMap<Stroke, Translation>);

#[derive(Debug, PartialEq, Eq)]
pub struct InvertedDictionary(HashMap<Translation, Vec<Stroke>>);

impl fmt::Display for Stroke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Dictionary {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path)?;

        serde_json::from_reader(file).map_err(Error::from)
    }

    pub fn invert(self) -> InvertedDictionary {
        let dict = self.0;

        let inverse = dict.into_iter().fold(
            HashMap::new(),
            |mut inverse: HashMap<_, Vec<_>>, (stroke, translation)| {
                inverse.entry(translation).or_default().push(stroke);
                inverse
            },
        );

        InvertedDictionary(inverse)
    }
}

impl InvertedDictionary {
    pub fn get(&self, translation: String) -> Option<&Vec<Stroke>> {
        self.0.get(&Translation(translation))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invert() {
        let dict = Dictionary(
            vec![
                (Stroke("TEFT".to_string()), Translation("test".to_string())),
                (Stroke("TEF".to_string()), Translation("test".to_string())),
            ]
            .into_iter()
            .collect(),
        );

        let expected = InvertedDictionary(
            vec![(
                Translation("test".to_string()),
                vec![Stroke("TEF".to_string()), Stroke("TEFT".to_string())],
            )]
            .into_iter()
            .collect(),
        );

        // Ensure order of items is predictable for comparison in assert_eq
        let mut inverted = dict.invert();
        inverted
            .0
            .iter_mut()
            .for_each(|(_, strokes)| strokes.sort());

        assert_eq!(expected, inverted);
    }
}
