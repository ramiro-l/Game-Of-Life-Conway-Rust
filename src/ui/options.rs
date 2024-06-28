use crossterm::event::KeyCode;

pub enum Option {
    PauseAndResume,
    NewRandomMap,
    Edit(u16, u16),
    Clear,
    Quit,
    Speed(SpeedOption),
    // Add more options here ...
    Any,
}

#[derive(PartialEq)]
pub enum SpeedOption {
    More,
    Less,
}

impl Option {
    pub fn from_key(key: KeyCode) -> Option {
        match key {
            KeyCode::Char(' ') => Option::PauseAndResume,
            KeyCode::Char('r') => Option::NewRandomMap,
            KeyCode::Char('q') => Option::Quit,
            KeyCode::Char('c') => Option::Clear,
            KeyCode::Char('+') => Option::Speed(SpeedOption::More),
            KeyCode::Char('-') => Option::Speed(SpeedOption::Less),
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
            Option::Edit(_, _) => "- 'pause&click' to edit.",
            Option::NewRandomMap => "- 'r' random map.",
            Option::Speed(_) => "- '+/-' control speed.",
            Option::Clear => "- 'c' to clear.",
            Option::Quit => "- 'q' to quit.",
            // Add messages for new options here ...
            Option::Any => "",
        }
    }

    pub fn all_messages() -> Vec<&'static str> {
        vec![
            Option::PauseAndResume.message(),
            Option::Edit(0, 0).message(),
            Option::Speed(SpeedOption::More).message(),
            Option::NewRandomMap.message(),
            Option::Clear.message(),
            Option::Quit.message(),
            // Add messages for new options here ...
        ]
    }
}
