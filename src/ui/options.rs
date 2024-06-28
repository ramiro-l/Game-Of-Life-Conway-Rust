use crossterm::event::KeyCode;

pub enum Option {
    PauseAndResume,
    NewRandomMap,
    Quit,
    // Add more options here ...
    Any,
}

impl Option {
    pub fn from_key(key: KeyCode) -> Option {
        match key {
            KeyCode::Char(' ') => Option::PauseAndResume,
            KeyCode::Char('r') => Option::NewRandomMap,
            KeyCode::Char('q') => Option::Quit,
            // Add key mapping for new options here ...
            _ => Option::Any,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Option::PauseAndResume => "- 'space' to pause/start.",
            Option::NewRandomMap => "- 'r' random map.",
            Option::Quit => "- 'q' to quit.",
            // Add messages for new options here ...
            Option::Any => "",
        }
    }

    pub fn all_messages() -> Vec<&'static str> {
        vec![
            Option::PauseAndResume.message(),
            Option::NewRandomMap.message(),
            Option::Quit.message(),
            // Add messages for new options here ...
        ]
    }
}
