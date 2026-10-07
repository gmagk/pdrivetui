use crate::APP_NAME;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{env, fs};
use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers};
use strum::IntoEnumIterator;
use crate::keybind::KeyBind;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub keybindings: KeyBindings

}

#[derive(Deserialize)]
#[serde(default)] // Automatically uses a default value from ::Default() when none is present
pub struct KeyBindings {
    pub quit: String,
    pub add: String,
    pub delete: String,
    pub download: String,
    pub info: String,
    pub rename: String,
    pub copy: String,
    pub unknown: String
}

impl Default for KeyBindings {

    fn default() -> Self {
        KeyBindings {
            quit: "Ctrl+q".to_string(),
            add: "a".to_string(),
            delete: "Ctrl+d".to_string(),
            download: "d".to_string(),
            info: "i".to_string(),
            rename: "r".to_string(),
            copy: "c".to_string(),
            unknown: String::new()
        }
    }
}

impl Config {

    // priority 1. Find file from cli option
    // priority 2. Find file from user directory
    // priority 3. Default values
    pub fn load(path: &str) -> Config {
        if Path::new(path).exists() {
            return Self::from_toml(path);
        } else {
            let home_path = Self::homedir_path();

            if Path::new(&home_path).exists() {
                return Self::from_toml(&home_path);
            }
        }

        Config {
            keybindings: KeyBindings::default()
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
                keybindings: KeyBindings::default()
            }
        })
    }

    fn homedir_path() -> String {
        let home_dir = env::home_dir().unwrap_or_else(|| PathBuf::default());

        format!("{}/.{}/config.toml", home_dir.to_str().unwrap(), APP_NAME.to_lowercase())
    }
}