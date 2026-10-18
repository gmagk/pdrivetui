use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;
use crate::dto::Type;
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
    CreateFolder,
    Copy,
    Delete,
    Download,
    EmptyTrash,
    Move,
    Paste,
    Place,
    Rename,
    Restore,
    Trash,
    Upload
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MyFiles => write!(f, "/my-files"),
            Self::SharedWithMe => write!(f, "/shared-with-me"),
            Self::Devices => write!(f, "/devices"),
            Self::Albums => write!(f, "/albums"),
            Self::Photos => write!(f, "/photos"),
            Self::PhotosSharedWithMe => write!(f, "/photos-shared-with-me"),
            Self::PhotosTrash => write!(f, "/photos-trash"),
            Self::Trash => write!(f, "/trash")
        }
    }
}

impl Path {

    pub fn top_capabilities(&self) -> Vec<PathCapability> {
        match self {
            Self::MyFiles => vec![CreateFolder, Download, Paste, Place, Upload],
            Self::SharedWithMe => vec![CreateFolder, Download, Paste, Place, Upload],
            Self::Devices => vec![CreateFolder, Download, Paste, Place, Upload],
            Self::Albums => vec![],
            Self::Photos => vec![],
            Self::PhotosSharedWithMe => vec![],
            Self::PhotosTrash => vec![],
            Self::Trash => vec![EmptyTrash]
        }
    }

    pub fn child_capabilities(&self, type_: &Type) -> Vec<PathCapability> {
        match self {
            Self::MyFiles => Self::add_type_specific(vec![Copy, Download, Move, Rename, Trash], type_),
            Self::SharedWithMe => Self::add_type_specific(vec![Copy, Download, Move, Rename, Trash], type_),
            Self::Devices => Self::add_type_specific(vec![Copy, Download, Move, Rename], type_),
            Self::Albums => vec![Rename],
            Self::Photos => vec![Rename, Trash],
            Self::PhotosSharedWithMe => vec![Rename],
            Self::PhotosTrash => vec![Rename],
            Self::Trash => vec![Delete, Restore]
        }
    }

    fn add_type_specific(mut c: Vec<PathCapability>, type_: &Type) -> Vec<PathCapability>{
        if type_ == &Type::Folder {
            c.push(CreateFolder);
            c.push(Paste);
            c.push(Place);
            c.push(Upload);
        }

        c
    }
}