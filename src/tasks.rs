use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

use crate::config;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    pub id: u32,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    // Have to get the latest entry to find new ID.
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, id, created_at }
    }
}

pub fn create_configuration_file() -> Result<()> {
  // Write TOML to ~/.config/cliban.toml
  // cliban_data = "~/.cliban.dat"
  // repaint = false
  Ok(())
}

pub fn show_board(config_path: PathBuf) -> Result<()> {
  let config = config::read_config(&config_path);
  println!("data at: {} - repaint: {}", config.cliban_data, config.repaint);
  Ok(())
}

pub fn add_task(config_path: PathBuf, task: Task) -> Result<()> {
  let config = config::read_config(&config_path);
  println!("data at: {} - repaint: {}", config.cliban_data, config.repaint);
  
  Ok(())
}

pub fn promote_task(config_path: PathBuf, id: u32) -> Result<()> {
  let config = config::read_config(&config_path);
  println!("data at: {} - repaint: {}", config.cliban_data, config.repaint);
  Ok(())
} 

pub fn regress_task(config_path: PathBuf, id: u32) -> Result<()> {
  let config = config::read_config(&config_path);
  println!("data at: {} - repaint: {}", config.cliban_data, config.repaint);
  Ok(())
} 

pub fn delete_task(config_path: PathBuf, id: u32) -> Result<()> {
  let config = config::read_config(&config_path);
  println!("data at: {} - repaint: {}", config.cliban_data, config.repaint);
  Ok(())
} 
