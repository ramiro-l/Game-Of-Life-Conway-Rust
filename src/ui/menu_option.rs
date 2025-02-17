use crossterm::event::KeyCode;

pub enum MenuOption {
    PauseAndResume,
    Edit(u16, u16),
    NewRandomMap,
    Speed(SpeedMenuOption),
    Clear,
    Quit,
    // Add more options here ...
    Any,
}

#[derive(PartialEq)]
pub enum SpeedMenuOption {
    More,
    Less,
}

impl MenuOption {
    pub fn from_key(key: KeyCode) -> MenuOption {
        match key {
            KeyCode::Char(' ') => MenuOption::PauseAndResume,
            // Edit option is 'pause&click' so it is not mapped here.
            KeyCode::Char('r') => MenuOption::NewRandomMap,
            KeyCode::Char('+') => MenuOption::Speed(SpeedMenuOption::More),
            KeyCode::Char('-') => MenuOption::Speed(SpeedMenuOption::Less),
            KeyCode::Char('c') => MenuOption::Clear,
            KeyCode::Char('q') => MenuOption::Quit,
            // Add key mapping for new options here ...
            _ => MenuOption::Any,
        }
    }

    pub fn from_mouse(column: u16, row: u16) -> MenuOption {
        return MenuOption::Edit(row, column);
    }

    fn message(&self) -> &'static str {
        match self {
            MenuOption::PauseAndResume => "- 'space' to pause/start.",
            MenuOption::Edit(_, _) => "- 'pause&click' to edit.",
            MenuOption::NewRandomMap => "- 'r' random map.",
            MenuOption::Speed(_) => "- '+/-' control speed.",
            MenuOption::Clear => "- 'c' to clear.",
            MenuOption::Quit => "- 'q' to quit.",
            // Add messages for new options here ...
            MenuOption::Any => "",
        }
    }

    pub fn all_messages() -> Vec<&'static str> {
        vec![
            MenuOption::PauseAndResume.message(),
            MenuOption::Edit(0, 0).message(),
            MenuOption::Speed(SpeedMenuOption::More).message(),
            MenuOption::NewRandomMap.message(),
            MenuOption::Clear.message(),
            MenuOption::Quit.message(),
            // Add messages for new options here ...
        ]
    }
}
