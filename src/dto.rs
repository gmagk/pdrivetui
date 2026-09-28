use serde::Deserialize;
use std::str::FromStr;

#[derive(Default, Clone,  Deserialize)]
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

#[derive(Default, Clone,  Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct RootDir {
    pub path: String
}

#[derive(Default, Clone,  Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct OkValue {
    pub ok: bool,
    pub value: String
}

#[derive(Default, Clone,  Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct OwnedBy {
    pub email: String
}

#[derive(Default, Clone,  Deserialize)]
#[serde(default)] // automatically use a default value when none is present in the data
pub struct ListElement {
    pub uid: String,

    #[serde(rename = "parentUid")]
    pub parent_uid: String,

    pub name: OkValue,

    #[serde(rename = "ownedBy")]
    pub owned_by: OwnedBy,

    pub type_: Type,

    #[serde(rename = "isShared")]
    pub is_shared: bool,

    #[serde(rename = "isSharedByUrl")]
    pub is_shared_by_url: String,

    #[serde(rename = "creationTime")]
    pub creation_time: String,

    // used for elements of type:file only
    #[serde(rename = "totalStorageSize")]
    pub total_storage_size: u64
}