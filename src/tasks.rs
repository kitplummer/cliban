use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;

use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;

use crate::config;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    pub state: String,
    pub id: u32,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: u32, text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        let state = "todo".to_string();
        Task { text, state, id, created_at }
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
  let expanded_path = shellexpand::tilde(&config.cliban_data);
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(expanded_path.into_owned())?;
  let tasks = collect_tasks(&file)?;
  println!("tasks: {:?}", tasks);
  
  Ok(())
}

pub fn add_task(config_path: PathBuf, new_task: String) -> Result<()> {
  let config = config::read_config(&config_path);
  let expanded_path = shellexpand::tilde(&config.cliban_data);
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(expanded_path.into_owned())?;

  let mut tasks = collect_tasks(&file)?;
  let id: u32 = (tasks.len() + 1) as u32;
  let task = Task::new(id, new_task);
  
  tasks.push(task);
  serde_json::to_writer(file, &tasks)?;

  if config.repaint {
    show_board(config_path)?;
  }
  Ok(())
}

pub fn promote_task(config_path: PathBuf, id: u32) -> Result<()> {
  let config = config::read_config(&config_path);
  let expanded_path = shellexpand::tilde(&config.cliban_data);
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(expanded_path.into_owned())?;

  let mut tasks = collect_tasks(&file)?;  let config = config::read_config(&config_path);
  let mut position: usize = 0;
  for i in 0..tasks.len() {
    if tasks[i].id == id {
      position = i;
    }
  }

  let task = &tasks[position];
  println!("{}: {}", task.id, task.state);

  let mut new_state = "";
  match task.state.as_str() {
    "todo" => new_state = "in-progress",
    "in-progress" => new_state = "done",
    "done" => println!("already done."),
    _ => ()
  }
  
  let new_text = &task.text;
  let mut new_task = Task::new(task.id, new_text.to_string());
  new_task.state = String::from(new_state);

  new_task.state = String::from(new_state);

  tasks.remove(position - 0);

  tasks.push(new_task);
  //tasks.push(task);
  serde_json::to_writer(file, &tasks)?;

  if config.repaint {
    show_board(config_path)?;
  }

  Ok(())
} 

pub fn regress_task(config_path: PathBuf, id: u32) -> Result<()> {
  let config = config::read_config(&config_path);
  println!("{}-> data at: {} - repaint: {}", id, config.cliban_data, config.repaint);
  Ok(())
} 

pub fn delete_task(config_path: PathBuf, id: u32) -> Result<()> {
  let config = config::read_config(&config_path);
  println!("{}-> data at: {} - repaint: {}", id, config.cliban_data, config.repaint);
  Ok(())
} 

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}