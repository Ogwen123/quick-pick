use crate::App;
use crate::widgets::list::List;
use crate::widgets::list_item::ListItem;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget};

pub struct Picker {}

impl Picker {
    pub fn draw(app: &mut App, frame: &mut Frame) {
        let list = List {
            items: app
                .pick_list_items
                .iter()
                .map(|x| ListItem {
                    name: x.name.clone(),
                    description: x.editor.clone() + " " + x.path.as_str(),
                })
                .collect::<Vec<ListItem>>(),
        };

        let vertical = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);

        frame.render_stateful_widget(
            list,
            frame.area().layout_vec(&vertical)[0],
            &mut app.pick_list_state,
        );

        Paragraph::new(Line::from(vec![
            "Quit".light_yellow(),
            " Esc".light_yellow().bold(),
            "Pick".light_yellow(),
            " Enter".light_yellow().bold(),
            "Edit".light_yellow(),
            " e".light_yellow().bold(),
        ]))
        .centered()
        .on_light_blue()
        .render(frame.area().layout_vec(&vertical)[1], frame.buffer_mut());
    }
}
