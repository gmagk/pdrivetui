use ratatui::widgets::ListState;

pub struct  ListWidget {}

impl ListWidget {

    pub fn next(total: &usize, list_state: &mut ListState) {
        let selected = list_state.selected().unwrap_or(0);
        let new = if selected == total - 1 { selected } else { selected + 1 };

        list_state.select(Some(new));
    }

    pub fn previous(list_state: &mut ListState) {
        let selected = list_state.selected().unwrap_or(0);
        let new = if selected > 0 { selected - 1 } else { selected };

        list_state.select(Some(new));
    }

    pub fn last(total: &usize, list_state: &mut ListState) {
        let new = total - 1;

        list_state.select(Some(new));
    }

    pub fn first(list_state: &mut ListState) {
        let new = 0;

        list_state.select(Some(new));
    }
}