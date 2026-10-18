use crate::dto::Type;
use crate::widget::content_widget::ContentWidget;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph, Wrap};

#[derive(Default)]
pub struct DetailsWidget {}

impl DetailsWidget {

    pub fn render(& self, frame: &mut Frame, area: Rect, content_widget: &ContentWidget) {
        let content_element = content_widget.selected();
        let path = content_element.parent().path();

        let mut text = Text::from(vec![
            Line::from(format!("Path: {}", if path.is_empty() { "/" } else { path.as_str() }))
        ]);

        if !content_element.is_top() {
            let created = content_element.created();
            if !created.is_empty() {
                text.push_line(Line::from(format!("Created: {}", created)));
            }

            let modified = content_element.modified();
            if !modified.is_empty() {
                text.push_line(Line::from(format!("Modified: {}", modified)));
            }
        }

        if content_element.TYPE == Type::File {
            text.push_line(Line::from(format!("Size: {}", content_element.size())))
        }

        if !content_element.is_empty {
            text.push_line(Line::from(format!("Type: {}", content_element.TYPE.to_string())));

            let shared = if content_element.is_shared { "Yes" } else { "No" };
            text.push_line(Line::from(format!("Shared: {}", shared)));

            let shared_by_url = if content_element.is_shared_by_url { "Yes" } else { "No" };
            text.push_line(Line::from(format!("Shared by URL: {}", shared_by_url)));

            if !content_element.owned_by.email.is_empty() {
                text.push_line(Line::from(format!("Owned by: {}", content_element.owned_by.email)));
            };
        }

        let content = Paragraph::new(text)
            .block(Block::bordered().title("Details"))
            .style(Style::new().white().on_black())
            .wrap(Wrap { trim: true });

        frame.render_widget(content, area);
    }
}