use crate::cli::Cli;
use crate::dto::{ListElement, Type};
use crate::keybind::KeyBind;
use crate::widget::content_widget::ContentWidget;
use crate::widget::list_widget::ListWidget;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListState};
use strum_macros::EnumIter;

#[derive(Default, Clone)]
pub enum ContextMenuItem {
    #[default]
    Unknown,
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
            Self::Cancel => String::from("Cancel"),
            Self::Unknown => String::from("")
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
    PendingPaste,
    PendingPlace,
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
            Self::PendingPaste => vec!(
                ContextMenuItem::Paste,
                ContextMenuItem::Cancel
            ),
            Self::PendingPlace => vec!(
                ContextMenuItem::Place,
                ContextMenuItem::Cancel
            ),
            Self::Unknown => vec![]
        }
    }

    pub fn find(
        widget: &ContextMenuWidget,
        content_widget: &ContentWidget,
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
            (c,cp) if widget.action_state == ContextMenuActionState::CopyPaste => Self::PendingPaste,
            // selected Move from a previous context menu
            (c,cp) if widget.action_state == ContextMenuActionState::MovePlace => Self::PendingPlace,
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

    // input for new file/folder name
    RenameInput,

    // selected to copy a listElement from previous action
    CopyPaste,

    // selected to move a listElement from previous action
    MovePlace,

    Deletion,

    Trashing,

    Restoration,

    TrashCleaning
}

#[derive(Default)]
pub struct ContextMenuWidget {
    pub menu: ContextMenu,
    pub list_state: ListState,
    pub action_state: ContextMenuActionState,
    pub action_state_context: String
}

impl ContextMenuWidget {

    pub fn check_state(&mut self, content_widget: &mut ContentWidget) {
        match self.action_state {
            ContextMenuActionState::CreateFolderInput => {
                if self.action_state_context.is_empty() {
                    return;
                }
                let new_folder_name = &self.action_state_context.replace(" ", "_");
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Cli::create_folder(&full_path, &new_folder_name);
                content_widget.request_sub_items(&content_widget.list_state.selected());
            },
            ContextMenuActionState::RenameInput => {
                if self.action_state_context.is_empty() {
                    return;
                }

                let new_name = &self.action_state_context.replace(" ", "_");
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Cli::rename(&full_path, &new_name);
                content_widget.refresh_items();
            }
            ContextMenuActionState::UploadInput => {
                if self.action_state_context.is_empty() {
                    return;
                }

                let source = &self.action_state_context.replace(" ", "_");
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Cli::upload(&full_path, &source);
                content_widget.refresh_items();
            }
            ContextMenuActionState::DownloadInput => {
                if self.action_state_context.is_empty() {
                    return;
                }

                let destination = &self.action_state_context;
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Cli::download(&full_path, &destination);
            }
            ContextMenuActionState::Deletion => {
                // TODO needs the file/folder name not the uid
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Cli::delete(&full_path);
                content_widget.refresh_items();
            }
            ContextMenuActionState::Trashing => {
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Cli::trash(&full_path);
                content_widget.refresh_items();
            }
            ContextMenuActionState::Restoration => {
                // TODO needs the file/folder name not the uid
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Cli::restore(&full_path);
                content_widget.refresh_items();
            }
            ContextMenuActionState::TrashCleaning => {
                let _ = Cli::empty_trash();
                content_widget.refresh_items();
            }
            ContextMenuActionState::MovePlace => {

            }
            ContextMenuActionState::CopyPaste => {

            }
            _ => {}
        }

        self.action_state = ContextMenuActionState::Unknown;

    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, content_widget: &ContentWidget) {
        self.menu = ContextMenu::find(&self, &content_widget, &content_widget.list_state);

        let widget = List::new(self.menu.items().iter().map(|item| item.name()).collect::<Vec<String>>())
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
            ListWidget::next(&self.menu.items().len(), &mut self.list_state);
        } else if KeyBind::is_nav_up(&key_event) {
            ListWidget::previous(&mut self.list_state);
        }
    }

    pub fn action(
        &mut self,
        loading_state: &mut bool,
        context_menu_state: &mut bool,
        context_menu_input_state: &mut bool
    ) {
        let items = self.menu.items();
        let item = items
            .get(self.list_state.selected().unwrap_or_default())
            .unwrap_or(&ContextMenuItem::Unknown);

        *context_menu_state = false;

        match item {
            ContextMenuItem::CreateFolder |
            ContextMenuItem::Upload |
            ContextMenuItem::Download |
            ContextMenuItem::Rename => {
                *context_menu_input_state = true
            }
            ContextMenuItem::Delete => {
                *loading_state = true;
                self.action_state = ContextMenuActionState::Deletion;
            }
            ContextMenuItem::Trash => {
                *loading_state = true;
                self.action_state = ContextMenuActionState::Trashing;
            }
            ContextMenuItem::EmptyTrash => {
                *loading_state = true;
                self.action_state = ContextMenuActionState::TrashCleaning;
            }
            ContextMenuItem::Restore => {
                *loading_state = true;
                self.action_state = ContextMenuActionState::Restoration;
            }
            ContextMenuItem::Copy => {

            }
            ContextMenuItem::Move => {

            }
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.list_state.select(Some(0));
    }
}


