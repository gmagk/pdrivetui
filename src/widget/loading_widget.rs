use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Clear, Padding, Paragraph};

#[derive(Default)]
pub struct LoadingWidget {}

impl LoadingWidget {

    pub fn render(frame: &mut Frame) {
        let area = frame
            .area()
            .centered(Constraint::Percentage(20), Constraint::Percentage(20));
        let content_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(10),
                Constraint::Fill(1),
            ])
            .split(area)[1];
        let middle = Layout::vertical([
                Constraint::Percentage(100),
            ])
            .split(content_area)[0];
        let content = Paragraph::new("Loading...")
            .block(Block::bordered())
            .centered()
            .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));

        frame.render_widget(Clear, content_area);
        frame.render_widget(content, middle);
    }
}