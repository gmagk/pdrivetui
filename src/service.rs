use crate::config::Strategy;
use crate::dto::{ListElement, TopDir};
use serde::de::DeserializeOwned;
use std::io::Error;
use std::process::{Command, Stdio};

pub struct Service {}

impl Service {

    pub fn login() {
        Self::exec_no_block("proton-drive auth login".to_string())
    }

    pub fn list_top() -> Result<Vec<TopDir>, Error> {
        Self::fs_action("list /".to_string())
    }

    pub fn create_folder(parent_path: &String, name: &String) -> Result<ListElement, Error> {
        Self::fs_action(format!("create-folder {} {}", parent_path, name))
    }

    pub fn rename(parent_path: &String, name: &String) -> Result<ListElement, Error> {
        Self::fs_action(format!("rename {} {}", parent_path, name))
    }

    pub fn upload(parent_path: &String, source: &String, strategy: &Strategy) -> Result<ListElement, Error> {
        Self::fs_action(format!("upload -f {} -d {} {} {}", strategy.upload_file, strategy.upload_folder, source, parent_path))
    }

    pub fn download(parent_path: &String, destination: &String, strategy: &Strategy) -> Result<ListElement, Error> {
        Self::fs_action(format!("download -f {} -d {} {} {}", strategy.download_file, strategy.download_folder, parent_path, destination))
    }

    pub fn list_dir(path: &String) -> Result<Vec<ListElement>, Error> {
        Self::fs_action(format!("list {}", path))
    }

    pub fn trash(path: &String) -> Result<Vec<TopDir>, Error> {
        Self::fs_action(format!("trash {}", path))
    }

    pub fn delete(path: &String) -> Result<Vec<TopDir>, Error> {
        Self::fs_action(format!("delete {}", path))
    }

    pub fn restore(path: &String) -> Result<Vec<TopDir>, Error> {
        Self::fs_action(format!("restore {}", path))
    }

    pub fn paste_to(source_path: &String, target_path: &String) -> Result<Vec<TopDir>, Error> {
        Self::fs_action(format!("copy {} {}", source_path, target_path))
    }

    pub fn move_to(source_path: &String, target_path: &String) -> Result<Vec<TopDir>, Error> {
        Self::fs_action(format!("move {} {}", source_path, target_path))
    }

    pub fn empty_trash() -> Result<Vec<TopDir>, Error> {
        Self::fs_action("empty-trash".to_string())
    }

    pub fn fs_action<T: DeserializeOwned>(subcommand: String) -> Result<T, Error> {
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

    fn exec_no_block(command: String) {
        let cmd = command.clone();
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect(format!("failed to execute `{}`", cmd).as_str())
            .stdout;
    }

    fn json_to_response<T>(source: Vec<String>) -> Result<T, Error>
    where T: DeserializeOwned {
        let str = source.iter().map(|x| x.to_string()).collect::<String>();

        match serde_json::from_str(str.as_str()) {
            Ok(t) => Ok(t),
            Err(e) => {
                Err(Error::other(format!("Error: {}\nResponse body: {}", e, str.as_str())))
            }
        }
    }
}