use crate::APP_NAME;
use crate::VERSION;
use clap::{Parser, Subcommand};

// Print help
#[derive(Parser)]
#[command(name = APP_NAME, version = VERSION, about = "A simple Proton Drive TUI", term_width = 80)] // term_width fixes large tab size
pub struct Help {
    #[command(subcommand)]
    command: Option<CliCommands>,
}

#[derive(Subcommand)]
enum CliCommands {
    /// Set configuration file
    ConfigFile {
        /// Path to configuration file
        path: String
    }
}