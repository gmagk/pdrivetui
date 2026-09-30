use crossterm::event::{KeyCode, KeyEvent};
use strum_macros::EnumIter;
use crate::config::Config;

#[derive(PartialEq, EnumIter)]
pub enum KeyBind {
    Quit,
    Add,
    Delete,
    Download,
    Info,
    Rename,
    Copy,
    Menu,
    Unknown
}

#[derive(PartialEq)]
pub enum StaticKeyBind {
    Down,
    Up,
    Right,
    Left,
    Unknown
}

impl KeyBind {

    pub fn name(&self) -> String {
        match self {
            Self::Quit => String::from("Quit"),
            Self::Add => String::from("Add"),
            Self::Delete => String::from("Delete"),
            Self::Download => String::from("Download"),
            Self::Info => String::from("Info"),
            Self::Rename => String::from("Rename"),
            Self::Copy => String::from("Copy"),
            Self::Menu => String::from("Menu"),
            _ => String::from("Unknown")
        }
    }

    pub fn value(&self, config: &Config) -> String {
        match self {
            Self::Quit => config.keybindings.quit.clone(),
            Self::Add => config.keybindings.add.clone(),
            Self::Delete => config.keybindings.delete.clone(),
            Self::Download => config.keybindings.download.clone(),
            Self::Info => config.keybindings.info.clone(),
            Self::Rename => config.keybindings.rename.clone(),
            Self::Copy => config.keybindings.copy.clone(),
            Self::Menu => config.keybindings.menu.clone(),
            _ => String::from("unknown")
        }
    }

    pub fn is_navigation(key_event: &KeyEvent) -> bool {
        Self::is_nav_down(key_event) ||
        Self::is_nav_up(key_event) ||
        Self::is_nav_right(key_event) ||
        Self::is_nav_left(key_event)
    }

    pub fn is_nav_down(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Char('j') || key_event.code == KeyCode::Down
    }

    pub fn is_nav_up(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Char('k') || key_event.code == KeyCode::Up
    }

    pub fn is_nav_right(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Char('l') || key_event.code == KeyCode::Right
    }

    pub fn is_nav_left(key_event: &KeyEvent) -> bool {
        key_event.code == KeyCode::Char('h') || key_event.code == KeyCode::Left
    }
}