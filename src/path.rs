use strum_macros::EnumIter;
use crate::path::PathCapability::{Copy, CreateFolder, Delete, Download, EmptyTrash, Move, Paste, Place, Rename, Restore, Trash, Upload};

#[derive(EnumIter)]
pub enum Path {
    MyFiles,
    SharedWithMe,
    Devices,
    Albums,
    Photos,
    PhotosSharedWithMe,
    PhotosTrash,
    Trash
}

pub enum PathCapability {
    Delete,
    Download,
    EmptyTrash,
    Copy,
    CreateFolder,
    Move,
    Paste,
    Place,
    Rename,
    Restore,
    Trash,
    Upload
}

impl Path {

    pub fn path(&self) -> String {
        match self {
            Self::MyFiles => "/my-files".to_string(),
            Self::SharedWithMe => "/shared-with-me".to_string(),
            Self::Devices => "/devices".to_string(),
            Self::Albums => "/albums".to_string(),
            Self::Photos => "/photos".to_string(),
            Self::PhotosSharedWithMe => "/photos-shared-with-me".to_string(),
            Self::PhotosTrash => "/photos-trash".to_string(),
            Self::Trash => "/trash".to_string()
        }
    }

    pub fn capabilities(&self) -> Vec<PathCapability> {
        match self {
            Self::MyFiles => vec![Download, Paste, CreateFolder, Place, Upload],
            Self::SharedWithMe => vec![Download, Paste, CreateFolder, Place, Upload],
            Self::Devices => vec![Download, Paste, CreateFolder, Place],
            Self::Albums => vec![],
            Self::Photos => vec![],
            Self::PhotosSharedWithMe => vec![],
            Self::PhotosTrash => vec![],
            Self::Trash => vec![EmptyTrash]
        }
    }

    pub fn child_capabilities(&self) -> Vec<PathCapability> {
        match self {
            Self::MyFiles => vec![Download, Copy, Paste, CreateFolder, Move, Place, Rename, Trash, Upload],
            Self::SharedWithMe => vec![Download, Copy, Paste, CreateFolder, Move, Place, Rename, Trash, Upload],
            Self::Devices => vec![Download, Copy, Paste, CreateFolder, Move, Place, Rename],
            Self::Albums => vec![Rename],
            Self::Photos => vec![Rename, Trash],
            Self::PhotosSharedWithMe => vec![Rename],
            Self::PhotosTrash => vec![Rename],
            Self::Trash => vec![Delete, Restore]
        }
    }
}