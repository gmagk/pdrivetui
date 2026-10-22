use ratatui::Frame;
use ratatui::prelude::Line;
use ratatui::symbols::border;
use ratatui::text::Text;
use ratatui::widgets::{Block, Paragraph};

#[derive(Default)]
pub struct LoginWidget {}

impl LoginWidget {
    pub fn render(frame: &mut Frame) {
        let mut text = Text::default();
        text.push_line(Line::from("Seems that you are not logged into Proton!"));
        text.push_line(Line::from(""));
        text.push_line(Line::from("Press [ Enter ] to login"));
        text.push_line(Line::from("Press [ Esc ] to exit"));
        let widget = Paragraph::new(text)
            .block(Block::bordered()
                .title(Line::from("Login").centered())
                .border_set(border::PLAIN)
            );

        frame.render_widget(widget, frame.area());
    }
}