use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Create the intial configuration in ~/.config/cliban.toml.
    Configure {
    },
    /// Display the kanban board.
    Show {
    },
    /// Create a new task in ToDo.
    Add {
        /// The task description text.
        #[structopt()]
        task: String
    },
    /// Promote the given task.
    Promote {
        #[structopt()]
        id: u32
    },
    /// Regress the given task
    Regress {
        #[structopt()]
        id: u32
    },
    /// Delete the given task
    Delete {
        #[structopt()]
        id: u32
    },
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cliban",
    about = "A command-line kanban board written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different configuration, other than default, file.
    #[structopt(parse(from_os_str), short, long)]
    pub config_file: Option<PathBuf>,
}
