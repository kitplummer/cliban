mod cli;
mod config;
mod tasks;

use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

use cli::{Action::*, CommandLineArgs};

fn find_default_config_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".config/cliban.toml");
        path
    })
}
fn main() -> anyhow::Result<()> {
    // Get the command-line args
    let CommandLineArgs {
        action,
        config_file,
    } = CommandLineArgs::from_args();

    let config_file = config_file
        .or_else(find_default_config_file)
        .ok_or(anyhow!("Failed to read default config file"))?;
        
    match action {
        Configure {} => tasks::create_configuration_file(),
        Show {} => tasks::show_board(config_file),
        Add { task } => tasks::add_task(config_file, task),
        Promote { id } => tasks::promote_task(config_file, id),
        Regress { id } => tasks::regress_task(config_file, id),
        Delete { id } => tasks::delete_task(config_file, id),
    }?;

    Ok(())
}
