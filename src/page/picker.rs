use crate::App;
use crate::pick::Pick;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::macros::span;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{List, ListState, Paragraph, Widget};

pub struct Picker {}

impl Picker {
    pub fn draw(app: &mut App, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);
        let [title_rect, list_rect, controls_rect] = area.layout(&vertical);

        Paragraph::new("Quick Pick")
            .centered()
            .bold()
            .on_magenta()
            .render(title_rect, frame.buffer_mut());

        Self::render_list(frame, list_rect, app);

        Paragraph::new(Line::from(vec![
            span!("Quit"),
            span!(" Esc").gray().bold(),
            span!(" • ").light_blue(),
            span!("Pick"),
            span!(" Enter").gray().bold(),
            span!(" • ").light_blue(),
            span!("Edit"),
            span!(" e").gray().bold(),
            span!(" • ").light_blue(),
            span!("New"),
            span!(" n").gray().bold(),
        ]))
        .centered()
        .gray()
        .render(controls_rect, frame.buffer_mut());
    }

    pub fn render_list(frame: &mut Frame, area: Rect, app: &mut App) {
        let mut str_items = app
            .pick_list_items
            .iter()
            .map(|x| x.text(&app.editors))
            .peekable();

        let mut spaced_items = Vec::with_capacity(str_items.len() * 2);

        while let Some(item) = str_items.next() {
            spaced_items.push(item);

            if str_items.peek().is_some() {
                spaced_items.push(Text::from(Line::from("\n")));
            }
        }

        let list = List::new(spaced_items)
            .style(Color::White)
            .highlight_style(Style::new().light_magenta().bold())
            .repeat_highlight_symbol(true)
            .highlight_symbol("│".light_blue().bold());

        frame.render_stateful_widget(list, area, &mut app.pick_list_state);
    }
}
