use crate::dto::{ListElement, Type};
use crate::widget::content_list_widget::ContentListWidget;
use crossterm::event::KeyEvent;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListState};
use strum_macros::EnumIter;

#[derive(Clone)]
pub enum ContextMenuItem {
    CreateFolder,
    Upload,
    Download,
    Rename,
    Copy,
    Paste,
    Move,
    Place,
    Trash,
    Restore,
    Delete,
    EmptyTrash,
    Cancel
}

impl ContextMenuItem {
    pub fn name(&self) -> String {
        match self {
            Self::CreateFolder => String::from("Create Folder"),
            Self::Upload => String::from("Upload"),
            Self::Download => String::from("Download"),
            Self::Rename => String::from("Rename"),
            Self::Copy => String::from("Copy"),
            Self::Paste => String::from("Paste Here"),
            Self::Move => String::from("Move"),
            Self::Place => String::from("Place Here"),
            Self::Trash => String::from("Trash"),
            Self::Restore => String::from("Restore"),
            Self::Delete => String::from("Delete"),
            Self::EmptyTrash => String::from("Empty Trash"),
            Self::Cancel => String::from("Cancel")
        }
    }
}

#[derive(Default, EnumIter)]
pub enum ContextMenu {
    #[default]
    Unknown,
    // keep these first, in order to be checked first in search loops: [
    Paste,
    Place,
    // ]
    Trash,
    InsideTrash,
    MyFiles,
    InsideMyFilesFolder,
    InsideMyFilesFile
}

impl ContextMenu {
    pub fn items(&self) -> Vec<ContextMenuItem> {
        match self {
            Self::Trash => vec!(
                ContextMenuItem::EmptyTrash
            ),
            Self::InsideTrash => vec!(
                ContextMenuItem::Restore,
                ContextMenuItem::Delete
            ),
            Self::MyFiles => vec!(
                ContextMenuItem::CreateFolder,
                ContextMenuItem::Upload
            ),
            Self::InsideMyFilesFolder => vec!(
                ContextMenuItem::CreateFolder,
                ContextMenuItem::Upload,
                ContextMenuItem::Rename,
                ContextMenuItem::Copy,
                ContextMenuItem::Move,
                ContextMenuItem::Trash
            ),
            Self::InsideMyFilesFile => vec!(
                ContextMenuItem::Download,
                ContextMenuItem::Rename,
                ContextMenuItem::Copy,
                ContextMenuItem::Move,
                ContextMenuItem::Trash
            ),
            Self::Paste => vec!(
                ContextMenuItem::Paste,
                ContextMenuItem::Cancel
            ),
            Self::Place => vec!(
                ContextMenuItem::Place,
                ContextMenuItem::Cancel
            ),
            Self::Unknown => vec![]
        }
    }

    pub fn find(
        widget: &ContextMenuListWidget,
        content_widget: &ContentListWidget,
        content_widget_state: &ListState
    ) -> Self {
        let content_state = content_widget_state.selected().unwrap_or_default();
        let content_element = content_widget.latest_results[content_state].clone();
        let content_parent_element: ListElement = if content_widget.current_path_elements.is_empty() {
            ListElement::default()
        } else {
            content_widget.current_path_elements[0].clone()
        };

        // When in top level, actions can be done only on /my-files and /trash elements.
        // When in sublevel, actions can be done only on /my-files and /trash subfolders or files.
        match (widget, content_element, content_parent_element) {
            // selected Trash
            (w,c,cp) if c.is_top_dir && c.top_dir_path == "/trash" => Self::Trash,
            // selected a Folder/File inside Trash
            (w,c,cp) if cp.top_dir_path == "/trash" => Self::InsideTrash,
            // selected MyFiles
            (w,c,cp) if c.is_top_dir && c.top_dir_path == "/my-files" => Self::MyFiles,
            // selected a Folder inside MyFiles
            (w,c,cp) if cp.top_dir_path == "/my-files" && c.type_ == Type::Folder => Self::InsideMyFilesFolder,
            // selected a File inside MyFiles
            (w,c,cp) if cp.top_dir_path == "/my-files" && c.type_ == Type::File => Self::InsideMyFilesFile,
            // selected Copy from a previous context menu
            (w,c,cp) if widget.copy_state => Self::Paste,
            // selected Move from a previous context menu
            (w,c,cp) if widget.move_state => Self::Place,
            _ => Self::Unknown
        }
    }
}

pub struct ContextMenuListWidget {
    pub selected_menu: ContextMenu,
    copy_state: bool,
    move_state: bool
}

impl ContextMenuListWidget {

    pub fn new() -> Self {
        Self {
            selected_menu: ContextMenu::Unknown,
            copy_state: false,
            move_state: false
        }
    }

    pub fn get_widget(&mut self, content_widget: &ContentListWidget, content_widget_state: &ListState) -> List {
        self.selected_menu = ContextMenu::find(&self, &content_widget, &content_widget_state);

        List::new(self.selected_menu.items().iter().map(|item| item.name()).collect::<Vec<String>>())
            .block(Block::default().title("Actions").borders(Borders::NONE))
            .style(Style::new().white().on_black())
            .highlight_style(Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::DarkGray)
                .bg(Color::DarkGray)
            )
    }

    pub fn act(&self, state: &ListState) {
        match self.selected_menu.items()[state.selected().unwrap_or_default()] {
            ContextMenuItem::CreateFolder => {},
            ContextMenuItem::Upload => {},
            ContextMenuItem::Download => {},
            ContextMenuItem::Rename => {},
            ContextMenuItem::Copy => {},
            ContextMenuItem::Paste => {},
            ContextMenuItem::Move => {},
            ContextMenuItem::Place => {},
            ContextMenuItem::Trash => {},
            ContextMenuItem::Restore => {},
            ContextMenuItem::Delete => {},
            ContextMenuItem::EmptyTrash => {},
            ContextMenuItem::Cancel => {}
        }
    }
}


