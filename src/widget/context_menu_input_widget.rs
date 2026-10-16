use crate::widget::context_menu_input_widget::ContextMenuInput::{DownloadDestination, NewName, NewSubFolder, Unknown, UploadSource};
use crate::widget::context_menu_widget::{ContextMenuActionState, ContextMenuItem, ContextMenuWidget};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};
use tui_input::Input;

#[derive(Default)]
pub enum ContextMenuInput {
    #[default]
    Unknown,
    NewSubFolder,
    NewName,
    UploadSource,
    DownloadDestination
}

impl ContextMenuInput {
    pub fn title(&self) -> String {
        match self {
            NewSubFolder => " New SubFolder ".to_string(),
            NewName => " New Name ".to_string(),
            UploadSource => " Upload Source ".to_string(),
            DownloadDestination => " Download Destination ".to_string(),
            Unknown => "".to_string()
        }
    }

    pub fn find(context_menu_widget: &ContextMenuWidget) -> Self {
        let context_menu_selected_item = context_menu_widget.selected();

        match context_menu_selected_item {
            ContextMenuItem::CreateFolder => NewSubFolder,
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
    menu_input: ContextMenuInput
}

impl ContextMenuInputWidget {

    pub fn render(&mut self, frame: &mut Frame, area: Rect, context_menu_widget: &ContextMenuWidget) {
        self.menu_input = ContextMenuInput::find(&context_menu_widget);

        let [input_area] = Layout::horizontal([Constraint::Percentage(50)])
            .flex(Flex::Center)
            .areas(area);
        let [input_area] = Layout::vertical([Constraint::Length(3)]) // keep 2 for borders and 1 for cursor
            .flex(Flex::Center)
            .areas(input_area);
        let width = input_area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let title = Line::from(vec![self.menu_input.title().bold()]);
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

    pub fn action(
        &mut self,
        loading_state: &mut bool,
        context_menu_input_state: &mut bool,
        context_menu_state: &mut bool,
        context_menu_widget: &mut ContextMenuWidget
    ) {
        *loading_state = true;
        *context_menu_input_state = false;
        *context_menu_state = false;

        match self.menu_input {
            NewSubFolder => {
                context_menu_widget.action_state = ContextMenuActionState::CreateFolderInput;
                context_menu_widget.action_state_context = self.input.value().to_string();
            },
            NewName => {
                context_menu_widget.action_state = ContextMenuActionState::RenameInput;
                context_menu_widget.action_state_context = self.input.value().to_string();
            },
            UploadSource => {
                context_menu_widget.action_state = ContextMenuActionState::UploadInput;
                context_menu_widget.action_state_context = self.input.value().to_string();
            },
            DownloadDestination => {
                context_menu_widget.action_state = ContextMenuActionState::DownloadInput;
                context_menu_widget.action_state_context = self.input.value().to_string();
            },
            _ => {}
        }
    }
}