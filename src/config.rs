use crate::APP_NAME;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    keybindings: KeyBindings

}

#[derive(Deserialize)]
pub struct KeyBindings {

    #[serde(default)]
    quit: String,

    #[serde(default)]
    add: String,

    #[serde(default)]
    del: String,

    #[serde(default)]
    download: String,

    #[serde(default)]
    info: String,

    #[serde(default)]
    rename: String,

    #[serde(default)]
    copy: String,

    #[serde(default)]
    menu: String

}

impl Default for KeyBindings {

    fn default() -> Self {
        KeyBindings {
            quit: "q".to_string(),
            add: "a".to_string(),
            del: "d".to_string(),
            download: "D".to_string(),
            info: "i".to_string(),
            rename: "r".to_string(),
            copy: "c".to_string(),
            menu: "m".to_string(),
        }
    }
}

impl Config {

    pub fn load(path: &str) -> Config {
        // priority1. Find file from cli option
        // priority2. Find file from user directory
        // priority 3. No file
        if Path::new(path).exists() {
            Self::from_toml(path)
        } else {
            let home_path = Self::homedir_path();

            if (Path::new(&home_path).exists()) {
                Self::from_toml(&home_path)
            } else  {
                Config {
                    keybindings: KeyBindings::default()
                }
            }
        }
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