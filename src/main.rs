use clap::{App, Arg};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    cliban_data: String,
    repaint: bool,
}

fn read_config(path: &str) {
    println!("CONFIG: {}", path);
    let file_contents = fs::read_to_string(path)
            .expect("Something went wrong reading the file");
    let config: Config = toml::from_str(&file_contents).unwrap();
    println!("cliban_data: {} - repaint: {}", config.cliban_data, config.repaint);
}

fn main() {

    let matches = App::new("cliban")
        .version("0.0.1")
        .author("Kit Plummer <kitplummer@gmail.com>")
        .about("Personal CLI Kanban")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(
            App::new("configure")
                .about("Sets up the default configuration file in ~/.config/cliban.toml")
        )
        .subcommand(
            App::new("show")
                .about("Shows the board")
        )
        .subcommand(
            App::new("add")
                .about("Creates a new task")
                .arg(
                    Arg::new("task")
                        .about("The task to add")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("promote")
                .about("Promotes a task")
                .arg(
                    Arg::new("id")
                        .about("The task id to promote")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("regress")
                .about("Regress a task")
                .arg(
                    Arg::new("id")
                        .about("The task id to regress")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("delete")
                .about("Deletes a task")
                .arg(
                    Arg::new("id")
                        .about("The task id to delete")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    if matches.is_present("configure") {
        println!("Creating config file in ~/.config/cliban.toml");
    }
    
    let home_dir = home::home_dir();

    let mut filename = home_dir
        .as_ref()
        .and_then(|name| name.to_str())
        .unwrap_or("default")
        .to_owned();

    let config_file: &str = "/.config/cliban.toml";
    
    filename.push_str(config_file);

    read_config(&filename);
    // You can check the value provided by positional arguments, or option arguments
    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(matches) = matches.subcommand_matches("add") {
        // "$ myapp test" was run
        if matches.is_present("task") {
            // "$ myapp test -l" was run
            println!("Adding task: {}", matches.value_of("task").unwrap());
        }
    }

    // Continued program logic goes here...
}
