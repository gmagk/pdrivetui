use serde::{Deserialize, Deserializer, Serialize};
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

    // proton returns this as OkValue struct
    #[serde(deserialize_with = "ListElement::deserialize_name")]
    pub name: String,

    #[serde(rename = "ownedBy")]
    pub owned_by: OwnedBy,

    #[serde(rename = "type")]
    pub TYPE: Type,

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

    // returned by Proton only on top directories
    pub path: String,

    pub parent: Option<Box<ListElement>>,

    pub is_empty: bool
}

impl ListElement {

    pub fn is_top(&self) -> bool {
        self.parent.is_none()
    }

    pub fn path(&self) -> String {
        let mut path = self.name.clone();
        let parent_opt = self.parent.as_ref();

        if parent_opt.is_none() {
            return path;
        }

        path = parent_opt.unwrap().path() + "/" + path.as_str();

        path
    }

    pub fn parent(&self) -> ListElement {
        if self.parent.is_none() {
            ListElement::default()
        } else {
            let p = self.parent.as_ref().unwrap();
            p.as_ref().clone()
        }
    }

    pub fn empty() -> ListElement {
        let mut el = ListElement::default();
        el.name = "<empty>".to_string();
        el.is_empty = true;

        el
    }

    pub fn deserialize_name<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ok_value = OkValue::deserialize(deserializer)?;

        Ok(if ok_value.ok { ok_value.value.to_string() } else { "".to_string() })
    }

}