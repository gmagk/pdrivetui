use crate::service::Service;
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
            Self::CreateFolder => String::from("Create SubFolder"),
            Self::Upload => String::from("Upload"),
            Self::Download => String::from("Download"),
            Self::Rename => String::from("Rename"),
            Self::Copy => String::from("Copy"),
            Self::Paste => String::from("Paste In Here"),
            Self::Move => String::from("Move"),
            Self::Place => String::from("Place In Here"),
            Self::Trash => String::from("Trash"),
            Self::Restore => String::from("Restore"),
            Self::Delete => String::from("Delete"),
            Self::EmptyTrash => String::from("Empty Trash"),
            Self::Cancel => String::from("Cancel"),
            Self::Unknown => String::from("")
        }
    }
}

#[derive(Default, PartialEq, EnumIter)]
pub enum ContextMenu {
    #[default]
    Unknown,
    PendingPaste,
    PendingPlace,
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
        // keep these first, in order to be checked first in search loops: [
            // selected Copy from a previous context menu, only on /my-files folder and subfolders
            (c,cp) if (c.top_dir_path == "/my-files" || cp.top_dir_path == "/my-files") && widget.action_state == ContextMenuActionState::ReadyToPaste => Self::PendingPaste,
            // selected Move from a previous context menu, only on /my-files folder and subfolders
            (c,cp) if (c.top_dir_path == "/my-files" || cp.top_dir_path == "/my-files") && widget.action_state == ContextMenuActionState::ReadyToPlace => Self::PendingPlace,
        // ]
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
    ReadyToPaste,

    Paste,

    // selected to move a listElement from previous action
    ReadyToPlace,

    Place,

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
                let _ = Service::create_folder(&full_path, &new_folder_name);
                content_widget.request_sub_items(&content_widget.list_state.selected());

                self.action_state = ContextMenuActionState::Unknown;
            },
            ContextMenuActionState::RenameInput => {
                if self.action_state_context.is_empty() {
                    return;
                }

                let new_name = &self.action_state_context.replace(" ", "_");
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Service::rename(&full_path, &new_name);
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::UploadInput => {
                if self.action_state_context.is_empty() {
                    return;
                }

                let source = &self.action_state_context.replace(" ", "_");
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Service::upload(&full_path, &source);
                content_widget.request_sub_items(&content_widget.list_state.selected());

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::DownloadInput => {
                if self.action_state_context.is_empty() {
                    return;
                }

                let destination = &self.action_state_context;
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Service::download(&full_path, &destination);

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::Deletion => {
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Service::delete(&full_path);
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::Trashing => {
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Service::trash(&full_path);
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::Restoration => {
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Service::restore(&full_path);
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::TrashCleaning => {
                let _ = Service::empty_trash();
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::Paste => {
                let source_path = &self.action_state_context;
                let target_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Service::paste_to(&source_path, &target_path);
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::Place => {
                let source_path = &self.action_state_context;
                let target_path = content_widget.full_path(&content_widget.list_state.selected());
                let _ = Service::move_to(&source_path, &target_path);
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            _ => {}
        }

    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        context_menu_state: &mut bool,
        content_widget: &ContentWidget
    ) {
        self.menu = ContextMenu::find(&self, &content_widget, &content_widget.list_state);

        if self.menu == ContextMenu::Unknown {
            *context_menu_state = false;

            return;
        }

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
        if KeyBind::is_down(&key_event) {
            ListWidget::next(&self.menu.items().len(), &mut self.list_state);
        } else if KeyBind::is_up(&key_event) {
            ListWidget::previous(&mut self.list_state);
        }
    }

    pub fn action(
        &mut self,
        loading_state: &mut bool,
        context_menu_state: &mut bool,
        context_menu_input_state: &mut bool,
        content_widget: &ContentWidget
    ) {
        let items = self.menu.items();
        let item = items
            .get(self.list_state.selected().unwrap_or_default())
            .unwrap_or(&ContextMenuItem::Unknown);
        let content_state = content_widget.list_state.selected().unwrap_or_default();

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
                self.action_state = ContextMenuActionState::ReadyToPaste;
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                self.action_state_context = full_path;
            }
            ContextMenuItem::Move => {
                self.action_state = ContextMenuActionState::ReadyToPlace;
                let full_path = content_widget.full_path(&content_widget.list_state.selected());
                self.action_state_context = full_path;
            }
            ContextMenuItem::Paste => {
                *loading_state = true;
                self.action_state = ContextMenuActionState::Paste;
            }
            ContextMenuItem::Place => {
                *loading_state = true;
                self.action_state = ContextMenuActionState::Place;
            }
            ContextMenuItem::Cancel => {
                self.reset_action_state()
            }
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.list_state.select(Some(0));
    }

    fn reset_action_state(&mut self) {
        self.action_state = ContextMenuActionState::Unknown;
        self.action_state_context = "".to_string();
    }
}


