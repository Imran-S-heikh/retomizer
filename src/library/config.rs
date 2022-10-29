use std::{collections::HashMap, path::Path, fs};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct Config {
    pub content: Vec<String>,
    pub breakpoints: HashMap<String, String>,
    pub custom: HashMap<String, String>,
    pub class_names: Vec<String>,
    pub exclude: Vec<String>,
}

impl Config {
    pub fn load(path: &Path) -> Self {
        let config = fs::read_to_string(path).unwrap();
        let config: Config = serde_json::from_str(&config).unwrap();

        config
    }
}
