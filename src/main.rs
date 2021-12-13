mod cli;
mod config;
mod tasks;

use anyhow::anyhow;
use structopt::StructOpt;

use config::*;

use cli::{Action::*, CommandLineArgs};


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
        Show { json } => tasks::show_board(config_file, json),
        Add { task } => tasks::add_task(config_file, task),
        Promote { id } => tasks::promote_task(config_file, id),
        Regress { id } => tasks::regress_task(config_file, id),
        Delete { id } => tasks::delete_task(config_file, id),
    }?;

    Ok(())
}
