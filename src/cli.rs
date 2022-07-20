use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Create the intial configuration in ~/.config/cliban.toml.
    Configure {
    },
    /// Display the kanban board (alias: s).
    #[structopt(alias = "s")]
    Show {
        /// Display as JSON instead of the default table
        #[structopt(short, long)]
        json: bool,
    },
    /// Create a new task in ToDo (alias: a).
    #[structopt(alias = "a")]
    Add {
        /// The task description text.
        #[structopt()]
        task: String
    },
    /// Promote the given task (alias: p).
    #[structopt(alias = "p")]
    Promote {
        #[structopt()]
        id: u32
    },
    /// Regress the given task (alias: r)
    #[structopt(alias = "r")]
    Regress {
        #[structopt()]
        id: u32
    },
    /// Delete the given task (alias: d)
    #[structopt(alias = "d")]
    Delete {
        #[structopt()]
        id: u32
    },
}

#[derive(Debug, StructOpt)]
#[structopt(after_help = "Thanks for checking out cliban!\nhttps://github.com/kitplummer/cliban")]
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
