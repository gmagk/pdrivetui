use crate::config::Config;
use crate::keybind::KeyBind;
use crate::widget::content_list_widget::ContentListWidget;
use crate::widget::context_menu_list_widget::ContextMenuListWidget;
use crate::widget::list_widget::ListWidget;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::Color;
use ratatui::style::Style;
use ratatui::widgets::{Block, ListState, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub struct App {
    config: Config,
    terminal: DefaultTerminal,
    content_widget: ContentListWidget,
    content_widget_state: ListState,
    context_menu_widget: ContextMenuListWidget,
    context_menu_widget_state: ListState,
    loading_state: bool,
    context_menu_state: bool
}

impl App {

    pub fn new(config: Config, terminal: DefaultTerminal) -> Self {
        Self {
            config,
            terminal,
            content_widget: ContentListWidget::new(),
            content_widget_state: ListState::default(),
            context_menu_widget: ContextMenuListWidget::new(),
            context_menu_widget_state: ListState::default(),
            loading_state: false,
            context_menu_state: false
        }
    }

    pub fn run(&mut self) -> io::Result<()>{
        self.content_widget_state.select(Some(0));
        show_loading(self);
        self.content_widget.request_top_items();

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

                // show context menu
                if !self.context_menu_state && keybind == KeyBind::ContextMenu {
                    show_context_menu(self);
                }

                // hide (all popups, ...)
                if key_event.code == KeyCode::Esc {
                    hide_popups(self);
                }

                // selection
                if key_event.code == KeyCode::Enter {
                    selection(self);
                }
            }
        }

        Ok(())
    }
}

fn draw(app: &mut App) {
    let _ = app.terminal.draw(|frame| {
        render(
            frame,
            &mut app.content_widget,
            &mut app.content_widget_state,
            &mut app.context_menu_widget,
            &mut app.context_menu_widget_state,
            &app.loading_state,
            &app.context_menu_state
        );
    });
}

fn render(
    frame: &mut Frame,
    content_widget: &mut ContentListWidget,
    content_widget_state: &mut ListState,
    context_menu_widget: &mut ContextMenuListWidget,
    context_menu_widget_state: &mut ListState,
    loading_state: &bool,
    context_menu_state: &bool
) {
    content_and_details(frame, content_widget, content_widget_state);

    if *loading_state {
        loading_popup(frame)
    }

    if *context_menu_state {
        context_menu_popup(frame, content_widget, content_widget_state, context_menu_widget, context_menu_widget_state)
    }
}

fn navigation(key_event: &KeyEvent, app: &mut App) {
    // context menu
    if app.context_menu_state {
        if KeyBind::is_nav_down(&key_event) {
            ListWidget::next(&app.context_menu_widget.selected_menu.items().len(), &mut app.context_menu_widget_state);
        } else if KeyBind::is_nav_up(&key_event) {
            ListWidget::previous(&mut app.context_menu_widget_state);
        }
        return;
    }

    // main content
    if KeyBind::is_nav_down(&key_event) {
        ListWidget::next(&app.content_widget.latest_results.len(), &mut app.content_widget_state);
    } else if KeyBind::is_nav_up(&key_event) {
        ListWidget::previous(&mut app.content_widget_state);
    } else if KeyBind::is_nav_right(&key_event) {
        show_loading(app);
        app.content_widget.request_sub_items(&app.content_widget_state.selected());
        app.content_widget_state.select(Some(0));
    } else if KeyBind::is_nav_left(&key_event) {
        show_loading(app);
        app.content_widget.request_super_items();
        app.content_widget_state.select(Some(0));
    }
}

fn selection(app: &mut App) {
    app.context_menu_widget.act(&app.context_menu_widget_state);
    app.context_menu_state = false;
    draw(app);
}

fn show_loading(app: &mut App) {
    app.loading_state = true;
    draw(app);
    app.loading_state = false;
}

fn show_context_menu(app: &mut App) {
    app.context_menu_state = true;
    app.context_menu_widget_state.select(Some(0));
    draw(app);
}

fn hide_popups(app: &mut App) {
    app.context_menu_state = false;
    // TODO add any other popup which requires manual hiding

    draw(app);
}

fn content_and_details(frame: &mut Frame, widget: &mut ContentListWidget, state: &mut ListState) {
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

    frame.render_stateful_widget(widget.get_widget(), layout[0], state);
    frame.render_widget(details, layout[1]);
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

fn context_menu_popup(
    frame: &mut Frame,
    content_widget: &mut ContentListWidget,
    content_widget_state: &mut ListState,
    context_menu_widget: &mut ContextMenuListWidget,
    context_menu_widget_state: &mut ListState
) {
    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Fill(2)
        ])
        .split(frame.area())[1];
    let widget = context_menu_widget.get_widget(&content_widget, &content_widget_state);

    frame.render_stateful_widget(widget, area, context_menu_widget_state);
}