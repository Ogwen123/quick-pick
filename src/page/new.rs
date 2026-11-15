use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::StatefulWidget;

pub struct NewPick {}

#[derive(Default)]
pub struct NewPickState {
    name: String,
    path: String,
}

impl NewPick {
    pub fn new() -> Self {
        NewPick {}
    }
}

impl StatefulWidget for NewPick {
    type State = NewPickState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {}
}
