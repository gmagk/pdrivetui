mod app;
mod cli;
mod utils;
mod help;
mod dto;
mod config;

use crate::app::App;
use crate::help::Help;
use clap::Parser;
use std::io;
use crate::config::Config;

const APP_NAME: &str = "DriveTUI";
const VERSION: &str = "1.0.0-alpha.1";

fn main() -> io::Result<()> {
    // Calling parse() makes Clap work
    Help::parse();

    let terminal = ratatui::init();
    let config = Config::load("");
    let result = App::new(config, terminal).run();

    ratatui::restore();

    result
}
