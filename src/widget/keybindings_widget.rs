use crate::config::Config;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Padding, Row, Table};

pub struct KeybindingsWidget {
    content: [Content; 10]
}

struct Content {
    key: String,
    binding: String,
    description: String,
    config: bool
}

impl KeybindingsWidget {

    pub fn new(config: &Config) -> Self {
        Self {
            content: [
                Content {
                    key: String::from("quit"),
                    binding: String::from(config.keybindings.quit.clone()),
                    description: String::from("close app"),
                    config: true
                },
                Content {
                    key: String::from("keybindings"),
                    binding: String::from(config.keybindings.keybindings.clone()),
                    description: String::from("list keybindings"),
                    config: true
                },
                Content {
                    key: String::from("move down"),
                    binding: String::from("j / Down"),
                    description: String::from("go 1 step down"),
                    config: false
                },
                Content {
                    key: String::from("move up"),
                    binding: String::from("k / Up"),
                    description: String::from("go 1 step up"),
                    config: false
                },
                Content {
                    key: String::from("move right"),
                    binding: String::from("l / Right"),
                    description: String::from("go 1 level down"),
                    config: false
                },
                Content {
                    key: String::from("move left"),
                    binding: String::from("h / Left"),
                    description: String::from("go 1 level up"),
                    config: false
                },
                Content {
                    key: String::from("page down"),
                    binding: String::from("Ctrl+d / PageDown"),
                    description: String::from("go to bottom"),
                    config: false
                },
                Content {
                    key: String::from("page up"),
                    binding: String::from("Ctrl+u / PageUp"),
                    description: String::from("go to top"),
                    config: false
                },
                Content {
                    key: String::from("select"),
                    binding: String::from("Enter"),
                    description: String::from("open context menu, e.t.c."),
                    config: false
                },
                Content {
                    key: String::from("close"),
                    binding: String::from("Esc"),
                    description: String::from("close context menu, e.t.c."),
                    config: false
                }
            ]
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let header_style = Style::new().fg(Color::Black);
        let headers = Row::new([
            Line::from(format!("  {}  ","action")).bold().style(header_style).left_aligned(),
            Line::from(format!("  {}  ","key")).bold().style(header_style).left_aligned(),
            Line::from(format!("  {}  ","usage")).bold().style(header_style).left_aligned(),
            Line::from(format!("  {}  ","config")).bold().style(header_style).centered()
        ]).bottom_margin(1);
        let rows = self.content.iter().enumerate().map(|(index, content)| {
            let bg = if index % 2 == 0 { Color::DarkGray } else { Color::Reset };
            let style = Style::default().bg(bg);

            Row::new([
                Line::from(format!("  {}  ",content.key.clone())).left_aligned().style(style),
                Line::from(format!("  {}  ",content.binding.clone())).left_aligned().style(style),
                Line::from(format!("  {}  ",content.description.clone())).left_aligned().style(style),
                Line::from(format!("  {}  ",if content.config { "*" } else { "" }.to_string())).centered().style(style)
            ])
        });

        let content_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([ Constraint::Length(15) ])
            .split(area)[0];
        let col_widths = [
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 1),
            Constraint::Ratio(1, 10)
        ];
        let table = Table::new(rows, col_widths)
            .header(headers)
            .block(Block::new().padding(Padding::new(2, 2, 1, 1)))
            .style(Style::default().fg(Color::Gray).bg(Color::Gray))
            .column_spacing(0);

        frame.render_widget(table, content_area);
    }
}