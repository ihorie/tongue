use std::collections::HashMap;

pub struct Config {
    pub aliase: HashMap<String, String>,
    pub variable: HashMap<String, String>,
    pub history: Vec<String>,
    pub home: String,
}
