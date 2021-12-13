extern crate exitcode;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub cliban_data: String,
    pub repaint: bool,
}

pub fn read_config(path: &Path) -> Config {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
    config
}