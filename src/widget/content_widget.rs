use url::Url;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::cli::Cli;
use crate::dto::{ListElement, TopDir, Type};
use ratatui::prelude::Style;
use ratatui::style::{Color, Modifier};
use ratatui::widgets::{Block, Borders, List, ListState};
use crate::keybind::KeyBind;
use crate::widget::list_widget::ListWidget;

#[derive(Default, PartialEq)]
enum ContentListActionState {
    #[default]
    Unknown,
    RequestSubItems,
    RequestSuperItems
}

#[derive(Default)]
pub struct ContentWidget {
    pub content: Vec<ListElement>,
    pub parents: Vec<ListElement>,
    pub list_state: ListState,
    action_state: ContentListActionState
}

impl ContentWidget {

    pub fn check_state(&mut self) {
        match self.action_state {
            ContentListActionState::RequestSubItems => {
                self.request_sub_items(& self.list_state.selected());
                self.list_state.select(Some(0));
            },
            ContentListActionState::RequestSuperItems => {
                self.request_super_items();
                self.list_state.select(Some(0));
            },
            _ => {}
        }

        self.action_state = ContentListActionState::Unknown;
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let print_values: Vec<String>;
        if self.parents.is_empty() {
            print_values = self.content
                .iter()
                .map(|list_element: &ListElement| list_element.top_dir_path.clone())
                .collect::<Vec<String>>()
        } else {
            print_values = self.content
                .iter()
                .map(|list_element: &ListElement| {
                    if list_element.name.ok {
                        list_element.name.value.clone()
                    } else {
                        String::new()
                    }
                }).collect::<Vec<String>>()
        }

        let widget = List::new(print_values)
            .block(Block::default().title(" Contents ").borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .highlight_style(Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Indexed(255)) // https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
                .bg(Color::Black)
            );

        frame.render_stateful_widget(widget, area, &mut self.list_state);
    }

    pub fn navigation(&mut self, key_event: &KeyEvent, loading_state: &mut bool) {
        self.action_state = ContentListActionState::Unknown;
        if KeyBind::is_nav_down(&key_event) {
            ListWidget::next(&self.content.len(), &mut self.list_state);
        } else if KeyBind::is_nav_up(&key_event) {
            ListWidget::previous(&mut self.list_state);
        } else if KeyBind::is_nav_pagedown(&key_event) {
            ListWidget::last(&self.content.len(), &mut self.list_state);
        } else if KeyBind::is_nav_pageup(&key_event) {
            ListWidget::first(&mut self.list_state);
        } else if KeyBind::is_nav_right(&key_event) {
            *loading_state = true;
            self.action_state = ContentListActionState::RequestSubItems;
        } else if KeyBind::is_nav_left(&key_event) {
            *loading_state = true;
            self.action_state = ContentListActionState::RequestSuperItems;
        }
    }

    pub fn action(&mut self, context_menu_state: &mut bool) {
        *context_menu_state = true
    }

    // converts TopDir DTOs to ListElements for unified later handling
    pub fn request_top_items(&mut self) {
        let result = Cli::list_top().unwrap_or_default();
        self.content = Vec::new();
        self.parents = Vec::new();
        let _ = result
            .iter()
            .for_each(|top_dir: &TopDir| {
                let mut list_element = ListElement::default();
                list_element.is_top_dir = true;
                list_element.top_dir_path = top_dir.path.clone();
                self.content.push(list_element);
            });
    }

    // this should not be called for the root dir (/)
    pub fn request_sub_items(&mut self, selected_index: &Option<usize>) {
        let index = selected_index.unwrap_or(0);
        let empty_element = ListElement::default();
        let list_element = self.content.get(index).unwrap_or(&empty_element);

        if !list_element.is_top_dir && list_element.type_ != Type::Folder {
            return;
        }

        let mut path: String;
        if list_element.is_top_dir {
            self.parents = vec![list_element.clone()];

            path = list_element.top_dir_path.clone();
        } else {
            self.parents.push(list_element.clone());

            path = self.parents[0].top_dir_path.clone();
            self.parents.iter().skip(1).for_each(|list_element: &ListElement| {
                path += "/";
                path += list_element.name.value.as_str();
            });
        }

        self.content = Cli::list_dir(&path).unwrap_or_default();
    }

    pub fn request_super_items(&mut self) {
        if self.parents.is_empty() {
            return;
        }

        self.parents.pop();

        if self.parents.is_empty() {
            self.request_top_items()
        } else {
            let mut path = self.parents[0].top_dir_path.clone();
            self.parents.iter().skip(1).for_each(|list_element: &ListElement| {
                path += "/";
                path += list_element.name.value.as_str();
            });

            self.content = Cli::list_dir(&path).unwrap_or_default();
        }
    }

    pub fn request_refresh_items(&mut self) {
        let mut path = String::new();

        // top level
        if self.parents.is_empty() {
            self.request_top_items();
            return;
        }

        // sublevel
        self.parents.iter().for_each(|list_element: &ListElement| {
            if list_element.is_top_dir {
                path += list_element.top_dir_path.as_str();
            } else {
                path += "/";
                path += list_element.name.value.as_str();
            }
        });

        self.content = Cli::list_dir(&path).unwrap_or_default();
    }

    pub fn full_path(&self, selected_index: &Option<usize>) -> String {
        let index = selected_index.unwrap_or(0);
        let mut path = String::new();
        let empty_element = &ListElement::default();
        let element = self.content.get(index).unwrap_or(empty_element);

        // top level
        if self.parents.is_empty() {
            return element.top_dir_path.clone();
        }

        // sublevel
        self.parents.iter().for_each(|parent: &ListElement| {
            let mut p = String::new();
            if parent.is_top_dir {
                p += parent.top_dir_path.to_string().as_str();
            } else {
                p += "/";
                p += parent.name.value.as_str();
            }
            path += p.as_str();
        });

        path += "/";
        path += self.content.get(index).unwrap_or(&empty_element).name.value.as_str();

        path
    }
}