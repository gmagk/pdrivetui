use crate::config::Config;
use crate::keybind::KeyBind;
use crate::widget::content_widget::ContentWidget;
use crate::widget::context_menu_input_widget::ContextMenuInputWidget;
use crate::widget::context_menu_widget::ContextMenuWidget;
use crate::widget::details_widget::DetailsWidget;
use crate::widget::keybindings_widget::KeybindingsWidget;
use crate::widget::loading_widget::LoadingWidget;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use tui_input::backend::crossterm::EventHandler;

pub struct App {
    config: Config,
    terminal: DefaultTerminal,
    details_widget: DetailsWidget,
    content_widget: ContentWidget,
    keybindings_widget: KeybindingsWidget,
    context_menu_widget: ContextMenuWidget,
    context_menu_input_widget: ContextMenuInputWidget,
    keybindings_state: bool,
    loading_state: bool,
    context_menu_state: bool,
    context_menu_input_state: bool
}

impl App {

    pub fn new(config: Config, terminal: DefaultTerminal) -> Self {
        let config_clone = config.clone();

        Self {
            config,
            terminal,
            content_widget: ContentWidget::default(),
            details_widget: DetailsWidget::default(),
            keybindings_widget: KeybindingsWidget::new(&config_clone),
            context_menu_widget: ContextMenuWidget::default(),
            context_menu_input_widget: ContextMenuInputWidget::default(),
            keybindings_state: false,
            loading_state: false,
            context_menu_state: false,
            context_menu_input_state: false
        }
    }

    pub fn run(&mut self) -> io::Result<()>{

        self.content_widget.request_top_items();

        loop {
            show_loader(self);

            self.render();

            // block and wait for user event
            let event = event::read()?;

            if let Event::Key(key_event, ..) = event {

                let keybind = self.config.find_keybind(&key_event);

                // quit
                if keybind == KeyBind::Quit {
                    break
                }

                // clear
                if key_event.code == KeyCode::Esc {
                    clear(self);
                    continue;
                }

                // action
                if key_event.code == KeyCode::Enter {
                    action(self);
                    continue;
                }

                // input
                if self.context_menu_input_state {
                    self.context_menu_input_widget.input.handle_event(&event);
                    continue;
                }

                // navigation
                if KeyBind::is_navigation(&key_event) {
                    navigation(self, &key_event);
                    continue;
                }

                // keybindings
                if keybind == KeyBind::Keybindings {
                    self.keybindings_state = true;
                    self.render()
                }
            }
        }

        Ok(())
    }

    fn render(&mut self) {
        let _ = self.terminal.draw(|frame| {
            render(
                frame,
                &mut self.content_widget,
                &mut self.details_widget,
                &mut self.keybindings_widget,
                &mut self.context_menu_widget,
                &mut self.context_menu_input_widget,
                &mut self.keybindings_state,
                &mut self.loading_state,
                &mut self.context_menu_state,
                & self.context_menu_input_state
            );
        });
    }
}

fn render(
    frame: &mut Frame,
    content_widget: &mut ContentWidget,
    details_widget: &mut DetailsWidget,
    keybindings_widget: &mut KeybindingsWidget,
    context_menu_widget: &mut ContextMenuWidget,
    context_menu_input_widget: &mut ContextMenuInputWidget,
    keybindings_state: &mut bool,
    loading_state: &mut bool,
    context_menu_state: &mut bool,
    context_menu_input_state: & bool
) {
    content_and_details(frame, content_widget, details_widget);

    if *keybindings_state {
        keybindings_widget.render(frame);
    }

    if *loading_state {
        LoadingWidget::render(frame);
    }

    if *context_menu_state {
        context_menu_widget.render(frame, context_menu_state, &content_widget);
    }

    if *context_menu_input_state {
        context_menu_input_widget.render(frame, &context_menu_widget);
    }
}

fn navigation(app: &mut App, key_event: &KeyEvent) {
    // keybindings
    if app.keybindings_state {
        app.keybindings_widget.navigation(&key_event);
        return;
    }

    // context menu
    if app.context_menu_state && !app.context_menu_input_state {
        app.context_menu_widget.navigation(&key_event);
        return;
    }

    // main content
    app.content_widget.navigation(&key_event, &mut app.loading_state);
}

fn action(app: &mut App) {
    if app.context_menu_input_state {
        app.context_menu_input_widget.action(
            &mut app.loading_state,
            &mut app.context_menu_input_state,
            &mut app.context_menu_state,
            &mut app.context_menu_widget
        )
    } else if app.context_menu_state {
        app.context_menu_widget.action(
            &mut app.loading_state,
            &mut app.context_menu_state,
            &mut app.context_menu_input_state,
            &mut app.content_widget
        );
    } else {
        app.content_widget.action(&mut app.context_menu_state);
    }
}

fn show_loader(app: &mut App) {

    if app.loading_state {
        clear(app);
        app.render();

        app.content_widget.check_state();
        app.context_menu_widget.check_state(&mut app.content_widget, &app.config.strategy);

        app.loading_state = false;
    }
}

fn clear(app: &mut App) {
    if app.context_menu_input_state {
        app.context_menu_input_widget.reset();
        app.context_menu_input_state = false;
    } else if app.context_menu_state {
        app.context_menu_widget.reset();
        app.context_menu_state = false;
    }
    app.keybindings_state = false;
}

fn content_and_details(
    frame: &mut Frame,
    content_widget: &mut ContentWidget,
    details_widget: & DetailsWidget
) {
    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());

    content_widget.render(frame, area[0]);
    details_widget.render(frame, area[1], content_widget);
}