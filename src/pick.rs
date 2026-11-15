use crate::App;
use std::fmt::{Display, Formatter};

pub struct Pick {
    pub name: String,
    pub path: String,
    pub editor: String,
}

impl Display for Pick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{} {}", self.name, self.editor, self.path)
    }
}

impl Pick {
    pub fn pick(&self, app: &mut bool) {}
}
