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
            // Add more options here ...
            _ => Option::Any,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Option::PauseAndResume => "- 'enter' to pause/resume.",
            Option::Quit => "- 'q' to quit.",
            Option::New => "- 'n' init new map.",
            // Add more options here ...
            Option::Any => "",
        }
    }
}

pub struct Menu {
    pub rows: u16,
    pub cols: u16,
    pub texts: Vec<&'static str>,
}

impl Menu {
    pub fn new() -> Menu {
        let texts = vec![
            super::TITLE,
            super::DESCRPTION,
            Option::Quit.message(),
            Option::New.message(),
            Option::PauseAndResume.message(),
            // Add more options here ...
        ];

        let (rows, cols) = Menu::calculate_rows_and_cols(&texts);

        Menu { rows, cols, texts }
    }

    fn calculate_rows_and_cols(texts: &Vec<&str>) -> (u16, u16) {
        let mut cols = texts.iter().map(|s| s.len()).max().unwrap() as u16;
        let mut rows = texts.len() as u16;

        // Add padding
        cols = cols + 4;
        rows = rows + 2;

        (rows, cols)
    }
}
