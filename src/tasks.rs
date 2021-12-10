use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;

use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;
use std::process;

use crate::config;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    pub state: String,
    pub id: u32,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: u32, text: String) -> Task {
        let updated_at: DateTime<Utc> = Utc::now();
        let state = "todo".to_string();
        Task { text, state, id, updated_at }
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
    } else {
      println!("Not task with id {} found", id);
      process::exit(1);
    }
  }
  let task = &tasks[position];
  let new_state = match task.state.as_str() {
    "todo" => "in-progress",
    "in-progress" => "done",
    "done" => {
      println!("Task {} already done.", task.id);
      process::exit(1);
    }
    _ => {
      println!("No task found.");
      process::exit(1);
    }
  };
  
  let new_text = &task.text;
  let mut new_task = Task::new(task.id, new_text.to_string());

  new_task.state = String::from(new_state);
  tasks.remove(position - 0);
  tasks.push(new_task);
  file.set_len(0)?;
  serde_json::to_writer(file, &tasks)?;

  if config.repaint {
    show_board(config_path)?;
  }

  Ok(())
} 

pub fn regress_task(config_path: PathBuf, id: u32) -> Result<()> {
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
    } else {
      println!("No task with id {} found.", id);
      process::exit(1);
    }
  }

  let task = &tasks[position];

  let new_state = match task.state.as_str() {
    "todo" => { 
      println!("Task {} already in todo.", task.id);
      process::exit(1);
    },
    "in-progress" => "todo",
    "done" => "in-progress", 
    _ => {
      println!("No task found.");
      process::exit(1);
    }
  };
  
  let new_text = &task.text;
  let mut new_task = Task::new(task.id, new_text.to_string());
  new_task.state = String::from(new_state);

  tasks.remove(position - 0);
  tasks.push(new_task);
  file.set_len(0)?;
  serde_json::to_writer(file, &tasks)?;

  if config.repaint {
    show_board(config_path)?;
  }

  Ok(())
} 

pub fn delete_task(config_path: PathBuf, id: u32) -> Result<()> {
  let config = config::read_config(&config_path);
  let expanded_path = shellexpand::tilde(&config.cliban_data);
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(expanded_path.into_owned())?;

  let mut tasks = collect_tasks(&file)?;  let config = config::read_config(&config_path);

  if tasks.is_empty() { 
    println!("No task with id {} found.", id);
    process::exit(1);
  }

  let mut position: usize = 0;
  for i in 0..tasks.len() {
    if tasks[i].id == id {
      position = i;
    } else {
      println!("No task with id {} found.", id);
      process::exit(1);
    }
  }

  tasks.remove(position - 0);

  file.set_len(0)?;
    // Write the modified task list back into the file;
  serde_json::to_writer(file, &tasks)?;

  if config.repaint {
    show_board(config_path)?;
  }
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