use crate::service::Service;
use crate::widget::login_url_widget::LoginUrlWidget;
use crate::widget::login_widget::LoginWidget;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::DefaultTerminal;

pub struct Init {
    popup_state: bool
}

impl Init {

    pub fn new() -> Self {
        Self {
            popup_state: false
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> bool {

        if Self::is_checked() {
            return true;
        }

        loop {

            Self::render(terminal, self.popup_state);

            // block and wait for user event
            let event = event::read().unwrap();

            if let Event::Key(key_event, ..) = event {

                if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
                    return false;
                }

                if key_event.code == KeyCode::Enter {
                    if self.popup_state {
                        return Self::is_checked();
                    } else {
                        self.login();
                    }
                }
            }
        }
    }

    fn is_checked() -> bool {
        Service::list_top().is_ok()
    }

    fn render(terminal: &mut DefaultTerminal, popup_state: bool) {
        let _ = terminal.draw(|frame| {
            LoginWidget::render(frame);
            if popup_state {
                LoginUrlWidget::render(frame);
            }
        });
    }

    fn login(&mut self) {
        self.popup_state = true;

        Service::login();
    }
}
