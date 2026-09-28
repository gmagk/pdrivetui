use crate::cli::Cli;
use crate::config::Config;
use crate::utils::utils;
use crossterm::event;
use crossterm::event::Event;
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub struct App {
    config: Config,
    terminal: DefaultTerminal
}

impl App {

    pub fn new(
        config: Config,
        terminal: DefaultTerminal
    ) -> Self {
        Self {
            config,
            terminal
        }
    }

    pub fn run(&mut self) -> io::Result<()>{
        loop {

            // block and wait for user event
            let event = event::read()?;

            if let Event::Key(key_event, ..) = event {

                // handle only keyboard keys
                if utils::key_events::is_keyboard_key(key_event) {
                    continue
                }

                // quit
                if utils::key_events::is_quit(key_event) {
                    break
                }
            }
        }

        Ok(())
    }
}