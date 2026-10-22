mod app;
mod service;
mod dto;
mod config;
mod keybind;
mod widget;
mod cli;
mod path;
mod init;

use crate::app::App;
use clap::Parser;
use std::io;
use crate::cli::Cli;
use crate::config::Config;
use crate::init::Init;

const APP_NAME: &str = "pdrivetui";
const VERSION: &str = "1.0.0-alpha.1";

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let cli = Cli::parse();
    let cli_config = cli.conf_file.as_path().to_str().unwrap();
    let config = Config::load(cli_config);
    if !Init::new().run(&mut terminal) {
        ratatui::restore();
        return Ok(());
    }
    let result = App::new(config, terminal).run();

    ratatui::restore();

    result
}
