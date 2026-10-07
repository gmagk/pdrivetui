use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    #[default]
    Unknown,
    Folder,
    File
}

impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "folder" => Ok(Type::Folder),
            "file" => Ok(Type::File),
            _ => Ok(Type::Unknown),
        }
    }
}

#[derive(Clone,  Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct TopDir {
    pub path: String
}

impl Default for TopDir {
    fn default() -> Self {
        Self { path: "/".to_string() }
    }
}


#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct OkValue {
    pub ok: bool,
    pub value: String
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct OwnedBy {
    pub email: String
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct ListElement {
    pub uid: String,

    #[serde(rename = "parentUid")]
    pub parent_uid: String,

    pub name: OkValue,

    #[serde(rename = "ownedBy")]
    pub owned_by: OwnedBy,

    #[serde(rename = "type")]
    pub type_: Type,

    #[serde(rename = "isShared")]
    pub is_shared: bool,

    #[serde(rename = "isSharedByUrl")]
    pub is_shared_by_url: bool,

    #[serde(rename = "creationTime")]
    pub creation_time: String,

    #[serde(rename = "modificationTime")]
    pub modification_time: String,

    // used for elements of type:file only
    #[serde(rename = "totalStorageSize")]
    pub total_storage_size: u64,

    // for top directories
    pub is_top_dir: bool,
    pub top_dir_path: String,

    pub nesting_level: i8
}