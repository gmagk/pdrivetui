use crate::command::Cli;
use crate::dto::{ListElement, TopDir, Type};
use ratatui::prelude::Style;
use ratatui::style::{Color, Modifier};
use ratatui::widgets::{Block, Borders, List, ListState};

pub struct ContentListWidget {
    pub latest_results: Vec<ListElement>,
    pub current_path_elements: Vec<ListElement>
}

impl ContentListWidget {

    pub fn new() -> Self {
        Self {
            latest_results: Vec::new(),
            current_path_elements: Vec::new()
        }
    }

    pub fn get_widget(&self) -> List {
        let print_values: Vec<String>;
        if self.current_path_elements.is_empty() {
            print_values = self.latest_results
                .iter()
                .map(|list_element: &ListElement| list_element.top_dir_path.clone())
                .collect::<Vec<String>>()
        } else {
            print_values = self.latest_results
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

    // converts TopDir DTOs to ListElements for unified later handling
    pub fn request_top_items(&mut self) {
        let result = Cli::list_top().unwrap_or_default();
        self.latest_results = Vec::new();
        self.current_path_elements = Vec::new();
        let _ = result
            .iter()
            .for_each(|top_dir: &TopDir| {
                let mut list_element = ListElement::default();
                list_element.is_top_dir = true;
                list_element.top_dir_path = top_dir.path.clone();
                self.latest_results.push(list_element);
            });
    }

    // this should not be called for the root dir (/)
    pub fn request_sub_items(&mut self, option: &Option<usize>) {
        let index = option.unwrap_or(0);
        let empty_element = ListElement::default();
        let list_element = self.latest_results.get(index).unwrap_or(&empty_element);

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
            self.current_path_elements.iter().skip(1).for_each(|list_element: &ListElement| {
                path += "/";
                path += list_element.uid.to_string().as_str();
            });
        }

        self.latest_results = Cli::list_dir(path).unwrap_or_default();
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

            self.latest_results = Cli::list_dir(path).unwrap_or_default();
        }
    }
}