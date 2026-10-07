use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::Style;
use ratatui::widgets::{Block, Paragraph, Wrap};

#[derive(Default)]
pub struct DetailsWidget {}

impl DetailsWidget {

    pub fn render(& self, frame: &mut Frame, area: Rect) {
        let content = Paragraph::new("")
            .block(Block::bordered().title("Details"))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(content, area);
    }
}