use serde::Serialize;

use crate::Stroke;

#[derive(Debug, Serialize)]
pub struct ScriptFilterResults {
    pub items: Vec<ScriptFilterItem>,
}

#[derive(Debug, Default, Serialize)]
pub struct ScriptFilterItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<ItemType>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    pub valid: bool,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum ItemType {
    Default,
    File,
    #[serde(rename = "file:skipcheck")]
    FileSkipcheck,
}

#[derive(Debug, Serialize)]
pub struct Icon {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<IconType>,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub enum IconType {
    Fileicon,
    Filetype,
}

pub fn results_to_alfred(results: &[&Stroke]) -> ScriptFilterResults {
    let items = results
        .iter()
        .map(|result| ScriptFilterItem {
            title: result.to_string(),
            valid: true,
            ..Default::default()
        })
        .collect();

    ScriptFilterResults { items }
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Default
    }
}
