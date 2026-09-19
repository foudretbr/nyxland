use clap::{Parser, Subcommand};
use nyx_ipc::NyxCommand;

#[derive(Parser, Debug)]
#[command(name = "nyxc", about = "Command line client for Nyxland Windows Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Switch active workspace
    Workspace { id: usize },
    /// Move focused window to workspace
    MoveToWorkspace { id: usize },
    /// Toggle floating state of focused window
    ToggleFloat,
    /// Force retile active workspace
    Retile,
    /// Focus next window
    FocusNext,
    /// Focus previous window
    FocusPrev,
    /// Reload configuration
    Reload,
    /// Stop Nyxland daemon
    Quit,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("Nyxland CLI dispatched command: {:?}", cli.command);
}
