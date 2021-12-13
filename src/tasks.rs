use chrono::{serde::ts_seconds, DateTime, Utc};
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;
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
  config::write_default_config().expect("Could not write default config file");
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

  let mut table = Table::new();
  let columns = get_columns(tasks);
  table
    .load_preset(UTF8_FULL)
    .set_content_arrangement(ContentArrangement::Dynamic)
    .set_header(vec![
      Cell::new("ToDo").fg(Color::Yellow),
      Cell::new("In-Progress").fg(Color::Green),
      Cell::new("Done").fg(Color::Red),
    ])
    .add_row(vec![
      columns.0,
      columns.1,
      columns.2,
    ]);
  println!("{}", table);
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
  
  if tasks.is_empty() { 
    println!("No task with id {} found.", id);
    process::exit(1);
  }
  
  let mut position: usize = 0;
  let mut found: bool = false;
  
  for (i, _) in tasks.iter().enumerate() {
    if tasks[i].id == id {
      position = i;
      found = true;
    }
  }

  if !found {
      println!("No task with id {} found", id);
      process::exit(1);
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
  tasks.remove(position);
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

  if tasks.is_empty() { 
    println!("No task with id {} found.", id);
    process::exit(1);
  }

  let mut position: usize = 0;
  let mut found: bool = false;
  for (i, _) in tasks.iter().enumerate() {
    if tasks[i].id == id {
      position = i;
      found = true;
    }
  }

  if !found {
      println!("No task with id {} found", id);
      process::exit(1);
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

  tasks.remove(position);
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
  let mut found: bool = false;

  for (i, _) in tasks.iter().enumerate() {
    if tasks[i].id == id {
      position = i;
      found = true;
    }
  }
  if !found {
      println!("No task with id {} found", id);
      process::exit(1);
  }
  tasks.remove(position);

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
        Err(e) => return Err(e.into()),
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

fn get_columns(tasks: Vec<Task>) -> (String, String, String) {

  let mut todos_string: String = String::new();
  let mut inprogs_string: String = String::new();
  let mut dones_string: String = String::new();

  for task in tasks {
    if task.state == "todo" {
      todos_string.push('[');
      todos_string.push_str(&task.id.to_string());
      todos_string.push_str("] ");
      todos_string.push_str(&task.text);
      todos_string.push('\n');
    }

    if task.state == "in-progress" {
      inprogs_string.push('[');
      inprogs_string.push_str(&task.id.to_string());
      inprogs_string.push_str("] ");
      inprogs_string.push_str(&task.text);
      inprogs_string.push('\n');
    } 

    if task.state == "done" {
      dones_string.push('[');
      dones_string.push_str(&task.id.to_string());
      dones_string.push_str("] ");
      dones_string.push_str(&task.text);
      dones_string.push('\n');
    } 

  }
  (
    todos_string.trim_end().to_string(),
    inprogs_string.trim_end().to_string(),
    dones_string.trim_end().to_string()
  )
}
