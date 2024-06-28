const TITLE: &str = "\x1B[1mConway Game of Life\x1B[0m";
const PADDING_T: u16 = 4;
const PADDING_L: u16 = 2;

pub struct Menu {
    pub rows: u16,
    pub cols: u16,
    pub texts: Vec<&'static str>,
}

impl Menu {
    pub fn new() -> Menu {
        let mut texts = vec![TITLE];
        texts.extend(super::Option::all_messages());

        let (rows, cols) = Menu::calculate_rows_and_cols(&texts);

        Menu { rows, cols, texts }
    }

    fn calculate_rows_and_cols(texts: &Vec<&str>) -> (u16, u16) {
        let mut cols = texts.iter().map(|s| s.len()).max().unwrap() as u16;
        let mut rows = texts.len() as u16;

        cols = cols + PADDING_T;
        rows = rows + PADDING_L;

        (rows, cols)
    }
}
