use crate::APP_NAME;
use crate::VERSION;
use std::path::PathBuf;
use clap::Parser;
use const_format::concatcp;

const CONF_FILE: &str = concatcp!("~/.", APP_NAME, "/conf.toml");

#[derive(Parser)]
#[command(name = APP_NAME, version = VERSION, about = "A simple Proton Drive TUI", term_width = 80)] // term_width fixes large tab size
pub struct Cli {

    /// Path to the config file
    #[arg(global = true, short, long, default_value = CONF_FILE)]
    pub conf_file: PathBuf
}