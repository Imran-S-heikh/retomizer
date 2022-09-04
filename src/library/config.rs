use std::collections::HashMap;

use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    content: Vec<String>,
    breakpoints: HashMap<String,String>,
    custom: HashMap<String,String>,
    class_names: Vec<String>,
    exclude: Vec<String>
}