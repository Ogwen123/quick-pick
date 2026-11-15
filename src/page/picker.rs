use crate::App;
use crate::pick::Pick;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::macros::span;
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{List, ListState, Paragraph, Widget};

pub struct Picker {}

impl Picker {
    pub fn draw(app: &mut App, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);
        let [title_rect, list_rect, controls_rect] = frame.area().layout(&vertical);

        Paragraph::new("Quick Pick")
            .centered()
            .bold()
            .light_green()
            .render(title_rect, frame.buffer_mut());

        Self::render_list(
            frame,
            list_rect,
            &mut app.pick_list_state,
            &app.pick_list_items,
        );

        Paragraph::new(Line::from(vec![
            "Quit".light_yellow(),
            " Esc".light_yellow().bold(),
            span!("    "),
            "Pick".light_yellow(),
            " Enter".light_yellow().bold(),
            span!("    "),
            "Edit".light_yellow(),
            " e".light_yellow().bold(),
            span!("    "),
            "New".light_yellow(),
            " n".light_yellow().bold(),
        ]))
        .centered()
        .on_light_blue()
        .render(controls_rect, frame.buffer_mut());
    }

    pub fn render_list(
        frame: &mut Frame,
        area: Rect,
        list_state: &mut ListState,
        items: &Vec<Pick>,
    ) {
        let str_items = items.iter().map(|x| x.to_string()).collect::<Vec<String>>();

        let list = List::new(str_items)
            .style(Color::White)
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, list_state);
    }
}
