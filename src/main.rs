mod app;
mod service;
mod dto;
mod config;
mod keybind;
mod widget;
mod cli;
mod path;

use crate::app::App;
use clap::Parser;
use std::io;
use crate::cli::Cli;
use crate::config::Config;

const APP_NAME: &str = "pdrivetui";
const VERSION: &str = "1.0.0-alpha.1";

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let cli = Cli::parse();
    let cli_config = cli.conf_file.as_path().to_str().unwrap();
    let config = Config::load(cli_config);
    let result = App::new(config, terminal).run();

    ratatui::restore();

    result
}
