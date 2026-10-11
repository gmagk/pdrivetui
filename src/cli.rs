use crate::dto::{ListElement, TopDir};
use serde::de::DeserializeOwned;
use std::io::Error;
use std::process::Command;

pub struct Cli {}

impl Cli {

    // TODO fix return types from service calls

    pub fn list_top() -> Result<Vec<TopDir>, Error> {
        Self::action("list /".to_string())
    }

    pub fn create_folder(parent_path: &String, name: &String) -> Result<ListElement, Error> {
        Self::action(format!("create-folder {} {}", parent_path, name))
    }

    pub fn rename(parent_path: &String, name: &String) -> Result<ListElement, Error> {
        Self::action(format!("rename {} {}", parent_path, name))
    }

    pub fn upload(parent_path: &String, source: &String) -> Result<ListElement, Error> {
        Self::action(format!("upload {} {}", source, parent_path))
    }

    pub fn download(parent_path: &String, destination: &String) -> Result<ListElement, Error> {
        Self::action(format!("download {} {}", parent_path, destination))
    }

    pub fn list_dir(path: &String) -> Result<Vec<ListElement>, Error> {
        Self::action(format!("list {}", path))
    }

    pub fn trash(path: &String) -> Result<Vec<TopDir>, Error> {
        Self::action(format!("trash {}", path))
    }

    pub fn delete(path: &String) -> Result<Vec<TopDir>, Error> {
        Self::action(format!("delete {}", path))
    }

    pub fn restore(path: &String) -> Result<Vec<TopDir>, Error> {
        Self::action(format!("restore {}", path))
    }

    pub fn paste_to(source_path: &String, target_path: &String) -> Result<Vec<TopDir>, Error> {
        eprintln!("{} --- {}", source_path, target_path);
        Self::action(format!("copy {} {}", source_path, target_path))
    }

    pub fn move_to(source_path: &String, target_path: &String) -> Result<Vec<TopDir>, Error> {
        Self::action(format!("move {} {}", source_path, target_path))
    }

    pub fn empty_trash() -> Result<Vec<TopDir>, Error> {
        Self::action("empty-trash".to_string())
    }

    pub fn action<T: DeserializeOwned>(subcommand: String) -> Result<T, Error> {
        Self::json_to_response::<T>(Self::exec(format!("proton-drive filesystem {} --json", subcommand)))
    }

    fn exec(command: String) -> Vec<String> {
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