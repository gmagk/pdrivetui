use ratatui::widgets::ListState;

pub struct  ListWidget {}

impl ListWidget {

    pub fn next(total: &usize, list_state: &mut ListState) {
        let max_index = total - 1;
        let i = match list_state.selected() {
            Some(i) => if i == max_index { i } else { i + 1 }
            None => 0,
        };
        list_state.select(Some(i));
    }

    pub fn previous(list_state: &mut ListState) {
        let i = match list_state.selected() {
            Some(i) => if i > 0 { i - 1 } else { i }
            None => 0,
        };
        list_state.select(Some(i));
    }
}