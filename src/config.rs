use crate::keybind::KeyBind;
use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use strum::IntoEnumIterator;

#[derive(Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub keybindings: KeyBindings,

    #[serde(default)]
    pub strategy: Strategy
}

#[derive(Deserialize, Clone)]
pub struct KeyBindings {
    pub quit: String,
    pub keybindings: String
}

#[derive(Deserialize, Clone)]
pub struct Strategy {
    pub upload_file: String,
    pub upload_folder: String,
    pub download_file: String,
    pub download_folder: String
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            quit: "Ctrl+q".to_string(),
            keybindings: "?".to_string()
        }
    }
}

impl Default for Strategy {
    fn default() -> Self {
        Strategy {
            upload_file: "create-new-revision".to_string(),
            upload_folder: "merge".to_string(),
            download_file: "rename".to_string(),
            download_folder: "merge".to_string()
        }
    }
}

impl Config {

    // priority 1. Find file from cli option (if missing then uses default user directory)
    // priority 2. Default values
    pub fn load(path: &str) -> Config {
        if Path::new(path).exists() {
            return Self::from_toml(path);
        }

        Config {
            keybindings: KeyBindings::default(),
            strategy: Strategy::default()
        }
    }

    pub fn find_keybind(&self, key_event: &KeyEvent) -> KeyBind {

        // handle only key-down events
        if key_event.kind != KeyEventKind::Press {
            return KeyBind::Unknown;
        }

        let has_ctrl = key_event.modifiers.contains(KeyModifiers::CONTROL);
        let has_alt = key_event.modifiers.contains(KeyModifiers::ALT);

        for key_bind in KeyBind::iter() {
            let val = key_bind.value(&self);

            if has_ctrl && !(val.contains("Ctrl+") || val.contains("ctrl+")) {
                continue;
            }
            if has_alt && !(val.contains("Alt+") || val.contains("alt+")) {
                continue;
            }

            let val = val
                .replace("Ctrl+", "")
                .replace("ctrl+", "")
                .replace("Alt+", "")
                .replace("alt+", "");
            if key_event.code.to_string() == val {
                return key_bind;
            }
        }

        KeyBind::Unknown
    }

    fn from_toml(path: &str) -> Config {
        let contents = fs::read_to_string(path)
            .unwrap_or_else(|e| {
                eprintln!("Failed to read config file: {e}");
                String::new()
            });

        toml::from_str(&contents).unwrap_or_else(|e| {
            eprintln!("Failed to parse config, using defaults: {e}");
            Config {
                keybindings: KeyBindings::default(),
                strategy: Strategy::default()
            }
        })
    }
}