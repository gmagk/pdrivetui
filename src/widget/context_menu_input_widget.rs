use crate::widget::context_menu_input_widget::ContextMenuInput::{DownloadDestination, NewFolder, NewName, Unknown, UploadSource};
use crate::widget::context_menu_list_widget::{ContextMenuItem, ContextMenuListWidget};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::{Block, ListState, Paragraph};
use tui_input::Input;

#[derive(Default)]
pub enum ContextMenuInput {
    #[default]
    Unknown,
    NewFolder,
    NewName,
    UploadSource,
    DownloadDestination
}

impl ContextMenuInput {
    pub fn title(&self) -> String {
        match self {
            NewFolder => " New Folder ".to_string(),
            NewName => " New Name ".to_string(),
            UploadSource => " Upload Source ".to_string(),
            DownloadDestination => " Download Destination ".to_string(),
            Unknown => "".to_string()
        }
    }

    pub fn find(
        context_menu_widget: &ContextMenuListWidget,
        context_menu_widget_state: &ListState
    ) -> Self {
        let context_menu_selected_index = context_menu_widget_state.selected().unwrap_or_default();
        let context_menu_selected_item = &context_menu_widget.state.items()[context_menu_selected_index].clone();

        match context_menu_selected_item {
            ContextMenuItem::CreateFolder => NewFolder,
            ContextMenuItem::Rename => NewName,
            ContextMenuItem::Upload => UploadSource,
            ContextMenuItem::Download => DownloadDestination,
            _ => Unknown
        }
    }
}

#[derive(Default)]
pub struct ContextMenuInputWidget {
    pub input: Input,
    state: ContextMenuInput
}

impl ContextMenuInputWidget {

    pub fn render(&mut self, frame: &mut Frame, area: Rect, context_menu_widget: &ContextMenuListWidget) {
        self.state = ContextMenuInput::find(&context_menu_widget, &context_menu_widget.list_state);

        let [input_area] = Layout::horizontal([Constraint::Percentage(50)])
            .flex(Flex::Center)
            .areas(area);
        let [input_area] = Layout::vertical([Constraint::Length(3)]) // keep 2 for borders and 1 for cursor
            .flex(Flex::Center)
            .areas(input_area);
        let width = input_area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let title = Line::from(vec![self.state.title().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::PLAIN);
        let widget = Paragraph::new(self.input.value())
            .scroll((0, scroll as u16))
            .block(block);

        frame.render_widget(widget, input_area);

        // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
        // end of the input text and one line down from the border to the input line
        let x = self.input.visual_cursor().max(scroll) - scroll + 1;

        frame.set_cursor_position((input_area.x + x as u16, input_area.y + 1));
    }

    pub fn reset(&mut self) {
        self.input.reset();
    }
}