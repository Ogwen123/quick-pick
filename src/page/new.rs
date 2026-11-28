use crossterm::event::Event;
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::StatefulWidget;
use crate::{App, Location};

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

    pub fn draw(app: &mut App, frame: &mut Frame, area: Rect) {
        if let Location::NewPick(state) = &mut app.location {
            Self::new().render(area, frame.buffer_mut(), state)
        }
    }

    pub fn handle_event(event: &Event, app: &mut App) {
        pub fn handle_event(event: &Event, app: &mut App) {
            let list_state = match &mut app.location {
                Location::NewPick(state) => state,
                _ => {panic!("Should not be possible, Location and event handler struct do not match.")}
            };
        }
    }
}

impl StatefulWidget for NewPick {
    type State = NewPickState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {}
}
