use crate::App;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::process::{Command, Stdio};

#[derive(Clone, Serialize, Deserialize)]
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
    pub fn pick(&self, app: &mut App) {
        let editor = match self.get_editor(&app.editors) {
            Some(res) => res,
            None => {
                app.error_string = Some(String::from("Could not build command for selected pick."));
                return;
            }
        };

        let mut cmd = match build_command(&editor.command, &self.path) {
            Ok(res) => res,
            Err(err) => {
                app.error_string = Some(format!("Could not build command for selected pick. {}", err));
                return
            }
        };


        cmd.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            cmd.process_group(0);
        }

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const DETACHED_PROCESS: u32 = 0x00000008;
            const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
            cmd.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
        }

        

        match cmd.spawn() {
            Ok(_) => app.exit = true,
            Err(err) => {
                app.error_string =
                    Some(format!("Could not run command for selected pick. {:?}", cmd));
                return;
            }
        }
    }

    pub fn text(&self, editors: &Vec<Editor>) -> Text {
        Text::from(vec![
            Line::from(self.name.clone()).bold(),
            Line::from(self.cmd(editors).unwrap_or("UNKNOWN".to_string()))
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

fn build_command(template: &String, path: &String) -> Result<Command, String> {
    let mut parts = template.split_whitespace();
    let program = parts.next().ok_or("Empty command")?;
    
    let resolved_program = which::which(program)
        .map_err(|e| format!("Could not find '{}': {}", program, e))?;

    let mut cmd = Command::new(resolved_program);

    for part in parts {
        if part == "<path>" {
            cmd.arg(path);
        } else if part.contains("<path>") {
            cmd.arg(part.replace("<path>", path));
        } else {
            cmd.arg(part);
        }
    }

    Ok(cmd)
}
