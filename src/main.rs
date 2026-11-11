mod page;
mod pick;
mod widgets;

use std::io;
use std::io::stderr;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph, StatefulWidget, Widget};
use ratatui::{DefaultTerminal, Frame};

use crate::pick::Pick;
use crate::widgets::list::{List, ListState};
use crate::widgets::list_item::ListItem;

use crate::page::picker::Picker;

pub enum Location {
    NewPick,
    Picker,
}

impl Default for Location {
    fn default() -> Self {
        Location::Picker
    }
}

#[derive(Default)]
pub struct App {
    location: Location,
    exit: bool,
    pick_list_items: Vec<Pick>,
    pick_list_state: ListState,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        match self.location {
            Location::Picker => Picker::draw(self, frame),
            Location::NewPick => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let event = event::read()?;

        match event {
            Event::Key(res) => match res.code {
                KeyCode::Esc => self.exit = true,
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    fn load_picks(&mut self) {
        self.pick_list_items = vec![Pick {
            name: String::from("test"),
            path: String::from("test"),
            editor: String::from("test"),
        }]
    }
}

fn main() -> io::Result<()> {
    stderr().execute(EnterAlternateScreen)?;
    ratatui::run(|terminal| App::default().run(terminal));
    stderr().execute(LeaveAlternateScreen)?;
    Ok(())
}
