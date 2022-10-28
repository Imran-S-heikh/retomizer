use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub content: Vec<String>,
    pub breakpoints: HashMap<String, String>,
    pub custom: HashMap<String, String>,
    pub class_names: Vec<String>,
    pub exclude: Vec<String>,
}
