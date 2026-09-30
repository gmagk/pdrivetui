use crate::config::Config;
use crate::keybind::KeyBind;
use crate::widget::list_widget::ListWidget;
use crossterm::event;
use crossterm::event::{Event, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::widgets::{Block, List, ListState, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use ratatui::prelude::Color;

pub struct App {
    config: Config,
    terminal: DefaultTerminal,
    list_widget: ListWidget,
    list_state: ListState,
    show_loading_popup: bool
}

impl App {

    pub fn new(config: Config, terminal: DefaultTerminal) -> Self {
        Self {
            config,
            terminal,
            list_widget: ListWidget::new(),
            list_state: ListState::default(),
            show_loading_popup: false
        }
    }

    pub fn run(&mut self) -> io::Result<()>{
        self.list_state.select(Some(0));
        show_loading_popup(self);
        self.list_widget.request_top_items();

        loop {

            draw(self);

            // block and wait for user event
            let event = event::read()?;

            if let Event::Key(key_event, ..) = event {

                let keybind = self.config.find_keybind(&key_event);

                // quit
                if keybind == KeyBind::Quit {
                    break
                }

                // navigation
                if KeyBind::is_navigation(&key_event) {
                    navigation(&key_event, self);
                }
            }
        }

        Ok(())
    }
}

fn draw(app: &mut App) {
    let _ = app.terminal.draw(|frame| {
        render(frame, app.list_widget.get_widget(), &mut app.list_state, &app.show_loading_popup);
    });
}

fn render(frame: &mut Frame, list: List, list_state: &mut ListState, show_loading_popup: &bool) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());

    let details = Paragraph::new("")
        .block(Block::bordered().title("Details"))
        .style(Style::new().white().on_black())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_stateful_widget(list, layout[0], list_state);
    frame.render_widget(details, layout[1]);

    if *show_loading_popup {
        loading_popup(frame)
    }
}

fn navigation(key_event: &KeyEvent, app: &mut App) {
    if KeyBind::is_nav_down(&key_event) {
        app.list_widget.next(&mut app.list_state);
    } else if KeyBind::is_nav_up(&key_event) {
        app.list_widget.previous(&mut app.list_state);
    } else if KeyBind::is_nav_right(&key_event) {
        show_loading_popup(app);
        app.list_widget.request_sub_items(&app.list_state.selected());
        app.list_state.select(Some(0));
    } else if KeyBind::is_nav_left(&key_event) {
        show_loading_popup(app);
        app.list_widget.request_super_items();
        app.list_state.select(Some(0));
    }
}

fn show_loading_popup(app: &mut App) {
    app.show_loading_popup = true;
    draw(app);
    app.show_loading_popup = false;
}

fn loading_popup(frame: &mut Frame) {
    let centered_area = frame
        .area()
        .centered(Constraint::Percentage(20), Constraint::Percentage(20));
    let paragraph_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(10),
            Constraint::Fill(1),
        ])
        .split(centered_area)[1];
    let paragraph = Paragraph::new("Loading...")
        .centered()
        .style(Style::default().fg(Color::DarkGray).bg(Color::DarkGray));
    frame.render_widget(paragraph, paragraph_area);
}