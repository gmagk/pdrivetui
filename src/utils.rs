pub mod utils {

    pub mod key_events {
        use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

        pub fn is_keyboard_key(event: KeyEvent) -> bool {
            event.kind != KeyEventKind::Press
        }
        
        pub fn is_quit(event: KeyEvent) -> bool {
            event.code == KeyCode::Char('q')
        }
    }
}