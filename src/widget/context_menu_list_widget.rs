use std::fmt::format;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::dto::{ListElement, Type};
use crate::widget::content_list_widget::ContentListWidget;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListState};
use strum_macros::EnumIter;
use crate::command::Cli;
use crate::keybind::KeyBind;
use crate::widget::list_widget::ListWidget;

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

    pub fn is_for_input(&self) -> bool {
        match self {
            Self::CreateFolder | Self::Upload | Self::Download => true,
            _ => false
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
        let content_element = content_widget.content[content_state].clone();
        let content_parent_element: ListElement = if content_widget.parents.is_empty() {
            ListElement::default()
        } else {
            content_widget.parents[0].clone()
        };

        // When in top level, actions can be done only on /my-files and /trash elements.
        // When in sublevel, actions can be done only on /my-files and /trash subfolders or files.
        match (content_element, content_parent_element) {
            // selected Trash
            (c,cp) if c.is_top_dir && c.top_dir_path == "/trash" => Self::Trash,
            // selected a Folder/File inside Trash
            (c,cp) if cp.top_dir_path == "/trash" => Self::InsideTrash,
            // selected MyFiles
            (c,cp) if c.is_top_dir && c.top_dir_path == "/my-files" => Self::MyFiles,
            // selected a Folder inside MyFiles
            (c,cp) if cp.top_dir_path == "/my-files" && c.type_ == Type::Folder => Self::InsideMyFilesFolder,
            // selected a File inside MyFiles
            (c,cp) if cp.top_dir_path == "/my-files" && c.type_ == Type::File => Self::InsideMyFilesFile,
            // selected Copy from a previous context menu
            (c,cp) if widget.action_state == ContextMenuActionState::Copy => Self::Paste,
            // selected Move from a previous context menu
            (c,cp) if widget.action_state == ContextMenuActionState::Move => Self::Place,
            _ => Self::Unknown
        }
    }
}

#[derive(Default, PartialEq)]
pub enum ContextMenuActionState {
    #[default]
    Unknown,

    // input for new folder name is opened from previous action
    CreateFolderInput,

    // input for upload destination folder path is opened from previous action
    UploadInput,

    // input for local destination folder path is opened from previous action
    DownloadInput,

    // selected to copy a listElement from previous action
    Copy,

    // selected to move a listElement from previous action
    Move
}

#[derive(Default)]
pub struct ContextMenuListWidget {
    pub state: ContextMenu,
    pub list_state: ListState,
    action_state: ContextMenuActionState,
    action_state_context: String
}

impl ContextMenuListWidget {

    pub fn render(&mut self, frame: &mut Frame, area: Rect, content_widget: &ContentListWidget) {
        self.state = ContextMenu::find(&self, &content_widget, &content_widget.list_state);

        let widget = List::new(self.state.items().iter().map(|item| item.name()).collect::<Vec<String>>())
            .block(Block::default().title(" Actions ").borders(Borders::NONE))
            .style(Style::new().white().on_black())
            .highlight_style(Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::DarkGray)
                .bg(Color::DarkGray)
            );

        frame.render_stateful_widget(widget, area, &mut self.list_state);
    }

    pub fn navigation(&mut self, key_event: &KeyEvent) {
        if KeyBind::is_nav_down(&key_event) {
            ListWidget::next(&self.state.items().len(), &mut self.list_state);
        } else if KeyBind::is_nav_up(&key_event) {
            ListWidget::previous(&mut self.list_state);
        }
    }

    pub fn action(&mut self, context_menu_input_state: &mut bool) {
        *context_menu_input_state = true
    }

    pub fn reset(&mut self) {
        self.list_state.select(Some(0));
    }

    pub fn action_MIGRATE_TO_INPUT(
        &self,
        context_menu_widget: &ContextMenuListWidget,
        context_menu_widget_state: &ListState,
        content_widget: &ContentListWidget,
        content_widget_state: &ListState,
        context_menu_input_state: &mut bool
    ) {
        let selected_menu_item = &self.state.items()[context_menu_widget_state.selected().unwrap_or_default()];
        let selected_content_item = &content_widget.content[content_widget_state.selected().unwrap_or_default()];
        let selected_content_item_full_path = content_widget.full_path(&content_widget_state.selected());

        match selected_menu_item {
            ContextMenuItem::CreateFolder => {
                // a new folder name is already given from previous action
                if  self.action_state == ContextMenuActionState::CreateFolderInput {
                    if !self.action_state_context.is_empty() {
                        let new_folder_name = &self.action_state_context;
                        Cli::create_folder(&selected_content_item_full_path, &new_folder_name);
                    }
                } // input for new folder's name
                else {

                }
            },
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


