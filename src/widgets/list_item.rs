use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

pub struct ListItem {
    pub name: String,
    pub description: String,
}

impl Widget for ListItem {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
