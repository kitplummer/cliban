use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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

pub fn write_default_config() -> Result<(), ()> {
    let config = Config {
        cliban_data: String::from("~/.cliban.json"),
        repaint: false,
    };

    let toml = toml::to_string(&config).unwrap();

    let config_path = find_default_config_file();
    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(config_path.unwrap())
    .unwrap();

    write!(&mut file, "{}", toml).expect("Could not write default configuration file");
    Ok(())
}

pub fn find_default_config_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".config/cliban.toml");
        path
    })
}