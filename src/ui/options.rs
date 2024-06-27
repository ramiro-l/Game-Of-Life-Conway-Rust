use crossterm::event::KeyCode;

pub enum Option {
    PauseAndResume,
    Quit,
    New,
    // Add more options here ...
    Any,
}

impl Option {
    pub fn from_key(key: KeyCode) -> Option {
        match key {
            KeyCode::Enter => Option::PauseAndResume,
            KeyCode::Char('q') => Option::Quit,
            KeyCode::Char('n') => Option::New,
            // Add key mapping for new options here ...
            _ => Option::Any,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Option::PauseAndResume => "- 'enter' to pause/resume.",
            Option::Quit => "- 'q' to quit.",
            Option::New => "- 'n' init new map.",
            // Add messages for new options here ...
            Option::Any => "",
        }
    }

    pub fn all_messages() -> Vec<&'static str> {
        vec![
            Option::PauseAndResume.message(),
            Option::Quit.message(),
            Option::New.message(),
            // Add messages for new options here ...
        ]
    }
}
