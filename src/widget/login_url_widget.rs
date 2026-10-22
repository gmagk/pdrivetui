use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Line;
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Clear, Paragraph};

#[derive(Default)]
pub struct LoginUrlWidget {}

impl LoginUrlWidget {

    pub fn render(frame: &mut Frame) {
        let area = frame
            .area()
            .centered(Constraint::Percentage(50), Constraint::Percentage(50));
        let content_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(30),
                Constraint::Fill(1),
            ]).split(area)[1];
        let middle = Layout::vertical([
                Constraint::Percentage(100),
            ]).split(content_area)[0];
        let mut text = Text::default();
        text.push_line(Line::from("If your Browser has not open then run"));
        text.push_line(Line::from("on terminal the command: `proton-drive auth login`"));
        text.push_line(Line::from(""));
        text.push_line(Line::from("Once completed the login"));
        text.push_line(Line::from("hit [ Enter ] to continue"));
        let content = Paragraph::new(text)
            .block(Block::bordered())
            .centered()
            .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));

        frame.render_widget(Clear, content_area);
        frame.render_widget(content, middle);
    }
}