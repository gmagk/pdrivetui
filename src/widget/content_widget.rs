use crate::dto::{ListElement, TopDir, Type};
use crate::keybind::KeyBind;
use crate::service::Service;
use crate::widget::list_widget::ListWidget;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::style::{Color, Modifier};
use ratatui::widgets::{Block, Borders, List, ListState};

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
    pub list_state: ListState,
    action_state: ContentListActionState
}

impl ContentWidget {

    pub fn check_state(&mut self) {
        match self.action_state {
            ContentListActionState::RequestSubItems => {
                self.request_sub_items();
            },
            ContentListActionState::RequestSuperItems => {
                let current_element = self.selected();

                if current_element.is_top() {
                    return;
                }

                let parent = current_element.parent();

                if parent.is_top() {
                    self.request_top_items()
                } else {
                    self.request_super_items();
                }
            },
            _ => {}
        }

        self.action_state = ContentListActionState::Unknown;
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let items = self.content
            .iter()
            .map(|list_element: &ListElement| list_element.name.clone())
            .collect::<Vec<String>>();

        let widget = List::new(items)
            .block(Block::default().title(" Contents ").borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .highlight_style(Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Indexed(255)) // https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
                .bg(Color::Black)
            );

        frame.render_stateful_widget(widget, area, &mut self.list_state);
    }

    pub fn selected(&self) -> &ListElement {
        let index = self.list_state.selected().unwrap_or_default();

        self.content.get(index).unwrap()
    }

    pub fn navigation(&mut self, key_event: &KeyEvent, loading_state: &mut bool) {
        self.action_state = ContentListActionState::Unknown;
        if KeyBind::is_down(&key_event) {
            ListWidget::next(&self.content.len(), &mut self.list_state);
        } else if KeyBind::is_up(&key_event) {
            ListWidget::previous(&mut self.list_state);
        } else if KeyBind::is_pagedown(&key_event) {
            ListWidget::last(&self.content.len(), &mut self.list_state);
        } else if KeyBind::is_pageup(&key_event) {
            ListWidget::first(&mut self.list_state);
        } else if KeyBind::is_right(&key_event) {
            *loading_state = true;
            self.action_state = ContentListActionState::RequestSubItems;
        } else if KeyBind::is_left(&key_event) {
            *loading_state = true;
            self.action_state = ContentListActionState::RequestSuperItems;
        }
    }

    pub fn action(&mut self, context_menu_state: &mut bool) {
        *context_menu_state = true
    }

    pub fn request_top_items(&mut self) {
        let result = Service::list_top().unwrap_or_default();

        let result_as_list_element = result
            .iter()
            .map(|top_dir: &TopDir| {
                let mut list_element = ListElement::default();
                list_element.name = top_dir.path.clone();

                // proton does not return any property for top directories other than `path`
                list_element.TYPE = Type::Folder;

                list_element
            }).collect::<Vec<ListElement>>();

        self.content = Self::parse_request_result(result_as_list_element, &None);
        self.list_state.select(Some(0));
    }

    // this should not be called for the root dir (/)
    pub fn request_sub_items(&mut self) {
        let current_element = self.selected();

        if current_element.is_empty || current_element.TYPE != Type::Folder {
            return;
        }

        let mut result = Service::list_dir(&current_element.path()).unwrap_or_default();
        if result.is_empty() {
            result = vec![ListElement::empty()];
        }

        self.content = Self::parse_request_result(result, &Some(Box::new(current_element.clone())));
        self.list_state.select(Some(0));
    }

    pub fn request_super_items(&mut self) {
        let current_element = self.selected();

        if current_element.is_top() {
            return;
        }

        let parent = current_element.parent();

        if parent.is_top() { // Should be checked and handled by the caller
            return;
        }

        let path = parent.parent().path();
        let result = Service::list_dir(&path).unwrap_or_default();

        self.content = Self::parse_request_result(result, &parent.parent);
        self.list_state.select(Some(0));
    }

    pub fn request_refresh_items(&mut self) {
        let current_element = self.selected();
        let path = if current_element.is_top() { "/".to_string() } else { current_element.parent().path() };

        let result = Service::list_dir(&path).unwrap_or_default();

        let parent = if current_element.is_top() { None } else { current_element.parent.clone() };
        self.content = Self::parse_request_result(result, &parent);
    }

    fn parse_request_result(result: Vec<ListElement>, parent: &Option<Box<ListElement>>) -> Vec<ListElement> {
        result
            .iter()
            .map(|list_element: &ListElement| {
                let mut el = list_element.clone();
                el.parent = parent.clone();

                el
            }).collect::<Vec<ListElement>>()
    }
}