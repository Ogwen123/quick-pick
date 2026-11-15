mod page;
mod pick;
mod widgets;

use std::io;
use std::io::stderr;
use std::time::{Duration, Instant};

use crate::page::new::{NewPick, NewPickState};
use crate::page::picker::Picker;
use crate::pick::Pick;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::widgets::{ListState, StatefulWidget};
use ratatui::{DefaultTerminal, Frame};

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
    pick_list_items: Vec<Pick>,
    pick_list_state: ListState,
    new_pick_state: NewPickState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            location: Location::Picker,
            exit: false,
            pick_list_items: Vec::new(),
            pick_list_state: ListState::default().with_selected(Some(0)),
            new_pick_state: NewPickState::default(),
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
        match self.location {
            Location::Picker => Picker::draw(self, frame),
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
                KeyCode::Up => self.pick_list_state.select_previous(),
                KeyCode::Down => self.pick_list_state.select_next(),
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
        self.pick_list_items = vec![
            Pick {
                name: String::from("test"),
                path: String::from("test"),
                editor: String::from("test"),
            },
            Pick {
                name: String::from("test2"),
                path: String::from("test2"),
                editor: String::from("test2"),
            },
            Pick {
                name: String::from("test3"),
                path: String::from("test3"),
                editor: String::from("test3"),
            },
            Pick {
                name: String::from("test4"),
                path: String::from("test4"),
                editor: String::from("test4"),
            },
            Pick {
                name: String::from("test5"),
                path: String::from("test5"),
                editor: String::from("test5"),
            },
        ]
    }
}

fn main() -> io::Result<()> {
    stderr().execute(EnterAlternateScreen)?;
    let _ = ratatui::run(|terminal| App::default().run(terminal));
    stderr().execute(LeaveAlternateScreen)?;
    Ok(())
}
