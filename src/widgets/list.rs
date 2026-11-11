use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::widgets::list_item::ListItem;

pub struct List {
    pub items: Vec<ListItem>,
}

pub struct ListState {
    page: u16,
    selected: String,
}

impl Default for ListState {
    fn default() -> Self {
        Self {
            page: 0,
            selected: String::new(),
        }
    }
}

impl StatefulWidget for List {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {}
}
