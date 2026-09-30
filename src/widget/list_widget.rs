use crate::cli::Cli;
use crate::dto::{ListElement, TopDir, Type};
use ratatui::prelude::Style;
use ratatui::style::{Color, Modifier};
use ratatui::widgets::{Block, Borders, List, ListState};

pub struct ListWidget {
    latest_result: Vec<ListElement>,
    current_path_elements: Vec<ListElement>
}

impl ListWidget {

    pub fn new() -> Self {
        Self {
            latest_result: Vec::new(),
            current_path_elements: Vec::new()
        }
    }

    pub fn get_widget(&self) -> List {
        let print_values: Vec<String>;
        if self.current_path_elements.is_empty() {
            print_values = self.latest_result
                .iter()
                .map(|list_element: &ListElement| list_element.top_dir_path.clone())
                .collect::<Vec<String>>()
        } else {
            print_values = self.latest_result
                .iter()
                .map(|list_element: &ListElement| {
                    if list_element.name.ok {
                        list_element.name.value.clone()
                    } else {
                        String::new()
                    }
                }).collect::<Vec<String>>()
        }

        List::new(print_values)
            .block(Block::default().title("Contents").borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .highlight_style(Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Indexed(255)) // https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
                .bg(Color::Black)
            )
    }

    pub fn next(&self, list_state: &mut ListState) {
        let max_index = self.latest_result.len() as i32 - 1;
        let i = match list_state.selected() {
            Some(i) => {
                if i as i32 == max_index {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        list_state.select(Some(i));
    }

    pub fn previous(&self, list_state: &mut ListState) {
        let i = match list_state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
            None => 0,
        };
        list_state.select(Some(i));
    }

    // converts TopDir DTOs to ListElements for unified later handling
    pub fn request_top_items(&mut self) {
        let result = Cli::list_top().unwrap_or_default();
        self.latest_result = Vec::new();
        self.current_path_elements = Vec::new();
        let _ = result
            .iter()
            .for_each(|top_dir: &TopDir| {
                let mut list_element = ListElement::default();
                list_element.is_top_dir = true;
                list_element.top_dir_path = top_dir.path.clone();
                self.latest_result.push(list_element);
            });
    }

    // this should not be called for the root dir (/)
    pub fn request_sub_items(&mut self, option: &Option<usize>) {
        let index = option.unwrap_or(0);
        let empty_element = ListElement::default();
        let list_element = self.latest_result.get(index).unwrap_or(&empty_element);

        if !list_element.is_top_dir && list_element.type_ != Type::Folder {
            return;
        }

        let mut path: String;
        if list_element.is_top_dir {
            self.current_path_elements = vec![list_element.clone()];

            path = list_element.top_dir_path.clone();
        } else {
            self.current_path_elements.push(list_element.clone());

            path = self.current_path_elements[0].top_dir_path.clone();
            self.current_path_elements.iter().for_each(|list_element: &ListElement| {
                path += "/";
                path += list_element.uid.to_string().as_str();
            });
        }

        self.latest_result = Cli::list_dir(path).unwrap_or_default();
    }

    pub fn request_super_items(&mut self) {
        if self.current_path_elements.is_empty() {
            return;
        }

        self.current_path_elements.pop();

        if self.current_path_elements.is_empty() {
            self.request_top_items()
        } else {
            let mut path = self.current_path_elements[0].top_dir_path.clone();
            self.current_path_elements.iter().skip(1).for_each(|list_element: &ListElement| {
                path += "/";
                path += list_element.uid.to_string().as_str();
            });

            self.latest_result = Cli::list_dir(path).unwrap_or_default();
        }
    }
}