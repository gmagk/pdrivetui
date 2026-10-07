use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::Paragraph;

#[derive(Default)]
pub struct LoadingWidget {}

impl LoadingWidget {

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let content_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(10),
                Constraint::Fill(1),
            ])
            .split(area)[1];
        let content = Paragraph::new("Loading...")
            .centered()
            .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));

        frame.render_widget(content, content_area);
    }
}