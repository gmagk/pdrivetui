use std::io::Error;
use std::process::Command;
use serde::de::DeserializeOwned;
use crate::dto::{ListElement, RootDir};

pub struct Cli {}

impl Cli {

    const BASE: &str = "proton-drive";
    const FS: &str = "filesystem";

    pub fn list_root() -> Result<Vec<RootDir>, Error> {
        Self::json_to_response(Self::command(format!("{} {} list / --json", Self::BASE, Self::FS)))
    }

    pub fn list_dir(path: String) -> Result<ListElement, Error> {
        Self::json_to_response(Self::command(format!("{} {} list {} --json", Self::BASE, Self::FS, path)))
    }

    fn command(command: String) -> Vec<String> {
        let cmd = command.clone();
        let status_command = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect(format!("failed to execute `{}`", cmd).as_str())
            .stdout;

        vec![String::from_utf8(status_command).unwrap_or_default()]
    }

    fn json_to_response<T>(source: Vec<String>) -> Result<T, Error>
    where T: DeserializeOwned {
        let str = source.iter().map(|x| x.to_string()).collect::<String>();

        match serde_json::from_str(str.as_str()) {
            Ok(t) => Ok(t),
            Err(e) => { Err(Error::other(format!("Error: {}\nResponse body: {}", e, str.as_str()))) }
        }
    }
}