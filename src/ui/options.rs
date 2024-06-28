use crossterm::event::KeyCode;

pub enum Option {
    PauseAndResume,
    NewRandomMap,
    Edit(u16, u16),
    Clear,
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
            KeyCode::Char('c') => Option::Clear,
            // Add key mapping for new options here ...
            _ => Option::Any,
        }
    }

    pub fn from_mouse(column: u16, row: u16) -> Option {
        return Option::Edit(row, column);
    }

    fn message(&self) -> &'static str {
        match self {
            Option::PauseAndResume => "- 'space' to pause/start.",
            Option::NewRandomMap => "- 'r' random map.",
            Option::Quit => "- 'q' to quit.",
            Option::Clear => "- 'c' to clear.",
            // Add messages for new options here ...
            Option::Any | Option::Edit(_, _) => "",
        }
    }

    pub fn all_messages() -> Vec<&'static str> {
        vec![
            Option::PauseAndResume.message(),
            "- 'pause&click' to edit.",
            Option::NewRandomMap.message(),
            Option::Clear.message(),
            Option::Quit.message(),
            // Add messages for new options here ...
        ]
    }
}
