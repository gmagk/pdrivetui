use crate::dto::Type;
use crate::keybind::KeyBind;
use crate::path::{Path, PathCapability};
use crate::service::Service;
use crate::widget::content_widget::ContentWidget;
use crate::widget::list_widget::ListWidget;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::{Offset, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, List, ListState, Padding, Shadow};
use strum::IntoEnumIterator;

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
            Self::Cancel => String::from("Cancel")
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
    pub items: Vec<ContextMenuItem>,
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
                let current_element = content_widget.selected();

                let _ = Service::create_folder(&current_element.path(), &new_folder_name);
                content_widget.request_sub_items();

                self.action_state = ContextMenuActionState::Unknown;
            },
            ContextMenuActionState::RenameInput => {
                if self.action_state_context.is_empty() {
                    return;
                }

                let new_name = &self.action_state_context.replace(" ", "_");
                let current_element = content_widget.selected();

                let _ = Service::rename(&current_element.path(), &new_name);
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::UploadInput => {
                if self.action_state_context.is_empty() {
                    return;
                }

                let source = &self.action_state_context.replace(" ", "_");
                let current_element = content_widget.selected();

                let _ = Service::upload(&current_element.path(), &source);
                content_widget.request_sub_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::DownloadInput => {
                if self.action_state_context.is_empty() {
                    return;
                }

                let destination = &self.action_state_context;
                let current_element = content_widget.selected();

                let _ = Service::download(&current_element.path(), &destination);

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::Deletion => {
                let current_element = content_widget.selected();

                let _ = Service::delete(&current_element.path());
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::Trashing => {
                let current_element = content_widget.selected();

                let _ = Service::trash(&current_element.path());
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::Restoration => {
                let current_element = content_widget.selected();

                let _ = Service::restore(&current_element.path());
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::TrashCleaning => {
                let _ = Service::empty_trash();
                content_widget.request_sub_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::Paste => {
                let source_path = &self.action_state_context;
                let current_element = content_widget.selected();

                let _ = Service::paste_to(&source_path, &current_element.path());
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            ContextMenuActionState::Place => {
                let source_path = &self.action_state_context;
                let current_element = content_widget.selected();

                let _ = Service::move_to(&source_path, &current_element.path());
                content_widget.request_refresh_items();

                self.action_state = ContextMenuActionState::Unknown;
            }
            _ => {}
        }

    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        context_menu_state: &mut bool,
        content_widget: &ContentWidget
    ) {
        let content_item = content_widget.selected();
        self.items = self.items_(content_item.path(), &content_item.TYPE);
        if self.items.is_empty() {
            *context_menu_state = false;

            return;
        }

        let widget = List::new(self.items.iter().map(|item| item.name()).collect::<Vec<String>>())
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM))
            .style(Style::new().white().on_black())
            .highlight_style(Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::DarkGray)
                .bg(Color::DarkGray)
            );

        let area = Rect::new(14, 1, 20, (self.items.len() + 1) as u16);

        frame.render_widget(Clear, area);
        frame.render_stateful_widget(widget, area, &mut self.list_state);
    }

    pub fn selected(&self) -> ContextMenuItem {
        let index = self.list_state.selected().unwrap_or_default();
        self.items[index].clone()
    }

    pub fn navigation(&mut self, key_event: &KeyEvent) {
        if KeyBind::is_down(&key_event) {
            ListWidget::next(&self.items.len(), &mut self.list_state);
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
        let item = self.items.get(self.list_state.selected().unwrap_or_default());
        if item.is_none() {
            return;
        }

        *context_menu_state = false;

        match item.unwrap() {
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
                self.action_state_context = content_widget.selected().path();
            }
            ContextMenuItem::Move => {
                self.action_state = ContextMenuActionState::ReadyToPlace;
                self.action_state_context = content_widget.selected().path();;
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

    // If an action_state is active then context_menu opens only on a path which supports capabilities
    // of this action_state, other paths will not show any context_menu.
    // E.g. When pending to paste then will show context_menu only on paths where we can paste.
    fn items_(&self, mut search_path: String, list_element_type: &Type) -> Vec<ContextMenuItem> {
        // must be of type /mypath, /mypath/subpath, ...
        if search_path.is_empty() || !search_path.starts_with('/') || search_path.len() < 2 {
            return Vec::new();
        }

        let is_top_path = search_path.chars().filter(|c| *c == '/').count() == 1;

        let capabilities = if is_top_path {
            match Path::iter().find(|p| p.path().eq(&search_path)) {
                Some(p) => p.capabilities(),
                None => return Vec::new()
            }
        } else {
            search_path.remove(0);
            let mut top_path = search_path.split_once('/').unwrap().0.to_string();
            top_path = format!("/{}", top_path);

            match Path::iter().find(|p| p.path().starts_with(&top_path)) {
                Some(p) => p.child_capabilities(),
                None => return Vec::new()
            }
        };

        let mut items = Vec::new();
        if self.action_state == ContextMenuActionState::ReadyToPaste {
            capabilities.iter().for_each(|c| match c {
                PathCapability::Paste => {
                    items.push(ContextMenuItem::Paste);
                    items.push(ContextMenuItem::Cancel);
                },
                _ => {}
            });
        } else if self.action_state == ContextMenuActionState::ReadyToPlace {
            capabilities.iter().for_each(|c| match c {
                PathCapability::Place => {
                    items.push(ContextMenuItem::Place);
                    items.push(ContextMenuItem::Cancel);
                },
                _ => {}
            });
        } else {
            capabilities.iter().for_each(|c| match c {
                PathCapability::Delete => items.push(ContextMenuItem::Delete),
                PathCapability::Download => items.push(ContextMenuItem::Download),
                PathCapability::EmptyTrash => items.push(ContextMenuItem::EmptyTrash),
                PathCapability::Copy => items.push(ContextMenuItem::Copy),
                PathCapability::CreateFolder => if *list_element_type == Type::Folder {
                    items.push(ContextMenuItem::CreateFolder)
                },
                PathCapability::Move => items.push(ContextMenuItem::Move),
                PathCapability::Rename => items.push(ContextMenuItem::Rename),
                PathCapability::Restore => items.push(ContextMenuItem::Restore),
                PathCapability::Trash => items.push(ContextMenuItem::Trash),
                PathCapability::Upload => if *list_element_type == Type::Folder {
                    items.push(ContextMenuItem::Upload)
                },
                _ => {}
            });
        }

        items
    }
}


