mod page;
mod pick;
mod utils;

use std::io;
use std::io::stderr;

use crate::page::new::{NewPick, NewPickState};
use crate::page::picker::Picker;
use crate::pick::{Editor, Pick};
use crate::utils::file::{File, read_file};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{ListState, Paragraph, StatefulWidget, Widget};
use ratatui::{DefaultTerminal, Frame};
use ratatui::text::{Line, Text};

pub enum Location {
    NewPick,
    Picker,
}

impl Default for Location {
    fn default() -> Self {
        Location::Picker
    }
}

pub struct App {
    location: Location,
    exit: bool,
    editors: Vec<Editor>,
    pick_list_items: Vec<Pick>,
    pick_list_state: ListState,
    new_pick_state: NewPickState,
    error_string: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            location: Location::Picker,
            exit: false,
            pick_list_items: Vec::new(),
            pick_list_state: ListState::default().with_selected(Some(0)),
            new_pick_state: NewPickState::default(),
            editors: Vec::new(),
            error_string: None,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.load_picks();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let remaining_area: Rect;

        match &self.error_string {
            Some(str) => {
                let vertical = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]);

                let [error_box, remaining] = frame.area().layout(&vertical);
                remaining_area = remaining;
                
                Paragraph::new(Text::from(vec![Line::from(""), Line::from(str.clone())]))
                    .centered()
                    .black()
                    .on_red()
                    .render(error_box, frame.buffer_mut())
            }
            None => remaining_area = frame.area(),
        }

        match self.location {
            Location::Picker => Picker::draw(self, frame, remaining_area),
            Location::NewPick => NewPick::render(
                NewPick::new(),
                frame.area(),
                frame.buffer_mut(),
                &mut self.new_pick_state,
            ),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let event = event::read()?;

        match event {
            Event::Key(res) if res.kind == KeyEventKind::Press => match res.code {
                KeyCode::Esc => self.exit = true,
                KeyCode::Up => self.pick_list_state.scroll_up_by(2),
                KeyCode::Down => self.pick_list_state.scroll_down_by(2),
                KeyCode::Enter => match self.pick_list_state.selected() {
                    Some(index) => match self.pick_list_items.get(index) {
                        Some(pick) => pick.pick(&mut self.exit),
                        None => {}
                    },
                    None => {}
                },
                KeyCode::Char('e') => self.exit = true,
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    fn load_picks(&mut self) {
        let data = match read_file(File::Picks) {
            Ok(res) => res,
            Err(err) => {
                self.error_string = Some(format!("Could not open picks file. {}", err));
                return;
            }
        };

        // split on ASCII 29, group seperator
        let split = data.split('\x1d').collect::<Vec<&str>>();

        if split.len() != 2 {
            self.error_string = Some(String::from("Corrupted or empty picks file."))
        }
        
        let raw_picks = split[0];
        let raw_editors = split[1];
        
        self.pick_list_items = match serde_json::from_str(raw_picks) {
            Ok(res) => res,
            Err(err) => {
                self.error_string = Some(format!("Failed to deserialise picks. {}", err));
                return;
            }
        };

        self.editors = match serde_json::from_str(raw_editors) {
            Ok(res) => res,
            Err(err) => {
                self.error_string = Some(format!("Failed to deserialise editors. {} {raw_editors}", err));
                return;
            }
        };
    }
}

fn main() -> io::Result<()> {
    stderr().execute(EnterAlternateScreen)?;
    let _ = ratatui::run(|terminal| App::default().run(terminal));
    stderr().execute(LeaveAlternateScreen)?;
    Ok(())
}
