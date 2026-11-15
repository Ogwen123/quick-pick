use crate::App;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct Pick {
    pub name: String,
    pub path: String,
    pub editor: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Editor {
    /// Used for linking to the editor in the `Pick`
    pub id: String,
    /// Display name
    pub name: String,
    /// Formatted as the command to run with the path denoted by <path> which will then be replaced by the path when running.
    pub command: String,
}

impl Pick {
    pub fn pick(&self, app: &mut bool) {}

    pub fn text(&self, editors: &Vec<Editor>) -> Text {
        Text::from(vec![
            Line::from(self.name.clone()).bold(),
            Line::from(format!(
                "{} {}\n",
                self.cmd(editors).unwrap_or("UNKNOWN".to_string()),
                self.path
            ))
            .gray()
            .dim(),
        ])
    }

    pub fn cmd(&self, editors: &Vec<Editor>) -> Option<String> {
        match self.get_editor(editors) {
            Some(editor) => Some((&editor.command.replace("<path>", self.path.as_ref())).clone()),
            None => None,
        }
    }

    pub fn get_editor(&self, editors: &Vec<Editor>) -> Option<Editor> {
        for editor in editors {
            if editor.id == self.editor {
                return Some(editor.clone());
            }
        }

        None
    }
}
