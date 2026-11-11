use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

pub struct NewPick {}

pub struct NewPickState {
    name: String,
    path: String,
}

impl StatefulWidget for NewPick {
    type State = NewPickState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {}
}
