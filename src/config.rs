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
    let default_data_path = 
        home::home_dir().map(|mut path| {
            path.push(".cliban.json");
            path
        })
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
        
    let config = Config {
        cliban_data: default_data_path,
        repaint: false,
    };

    let toml = toml::to_string(&config).unwrap();

    let config_path = find_default_config_file().unwrap();
    let prefix = config_path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let config_path = 
        config_path
        .into_os_string()
        .into_string()
        .unwrap();

    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(config_path)
    .unwrap();

    write!(&mut file, "{}", toml).expect("Could not write default configuration file");
    Ok(())
}

pub fn find_default_config_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".config");
        path.push("cliban.toml");
        path
    })
}