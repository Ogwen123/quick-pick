struct Config {
    default_editor: String,
}

impl Config {
    fn load() -> Self {
        Self {
            default_editor: String::new(),
        }
    }
}
