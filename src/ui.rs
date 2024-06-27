use crossterm::terminal::size;
use std::io::Write;

use crate::world::WorldMap;

pub const PADDING_W: u16 = 2;
pub const PADDING_H: u16 = 4;
const CHAR_POINT: &str = "██";
const CHAR_LINE_W_B: &str = "▄";
const CHAR_LINE_W_T: &str = "▀";
const CHAR_LINE_H_R: &str = "▌";
const CHAR_LINE_H_L: &str = "▐";

pub struct UserInterface {
    rows: u16,
    cols: u16,
    menu: Menu,
}

impl UserInterface {
    pub fn new() -> UserInterface {
        let menu = Menu::new();
        let (rows, cols) = terminal_size();

        UserInterface { rows, cols, menu }
    }

    pub fn print(&self) {
        std::io::stdout().flush().unwrap();
    }

    fn move_cursor(&self, row: u16, col: u16) {
        print!("\x1B[{};{}H", row, col);
    }

    fn draw_on(&self, row: u16, col: u16, c: &str) {
        self.move_cursor(row, col);
        print!("{}", c);
    }

    pub fn draw_point(&self, row: u16, mut col: u16) {
        // Check if the point is inside the menu
        if row < self.menu.rows && col < self.menu.cols {
            return;
        }

        if col % 2 == 1 {
            col = col + 1;
        };
        self.draw_on(row + PADDING_W, col + PADDING_H, CHAR_POINT);
    }

    fn draw_board(&self, rows: u16, cols: u16) {
        for i in 0..cols {
            self.draw_on(0, i, CHAR_LINE_W_T);
            self.draw_on(rows, i, CHAR_LINE_W_B);
        }

        for i in 0..rows + 1 {
            self.draw_on(i, 0, CHAR_LINE_H_L);
            self.draw_on(i, cols, CHAR_LINE_H_R);
        }
    }

    pub fn draw_border(&self) {
        self.draw_board(self.rows, self.cols);
    }

    pub fn draw_menu(&self) {
        // Add padding
        let x = 3;
        let y = 2;

        for i in 0..self.menu.texts.len() {
            self.draw_on(y + (i as u16), x, self.menu.texts[i as usize]);
        }

        for i in 2..self.menu.cols {
            self.draw_on(self.menu.rows, i, CHAR_LINE_W_B);
        }
        for i in 1..self.menu.rows + 1 {
            self.draw_on(i, self.menu.cols, "█");
        }
    }

    pub fn draw_map(&self, map: &WorldMap) {
        for (i, row) in map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let col = j as u16 * 2;
                let row = i as u16;
                if cell.is_alive() {
                    self.draw_point(row, col);
                }
            }
        }
    }

    pub fn cursor_hidden(&self) {
        print!("\x1B[?25l");
        self.print();
    }

    pub fn clear_screen(&self) {
        print!("\x1B[2J");
        self.print();
    }

    pub fn get_terminal_size(&self) -> (u16, u16) {
        (self.rows, self.cols)
    }
}

struct Menu {
    rows: u16,
    cols: u16,
    texts: Vec<&'static str>,
}

impl Menu {
    fn new() -> Menu {
        let texts = vec!["Press 'r + Enter' to restart", "Press 'q + Enter' to quit"];

        let cols = texts.iter().map(|s| s.len()).max().unwrap() as u16 + 4;
        let rows = texts.len() as u16 + 2;

        Menu { rows, cols, texts }
    }
}

fn terminal_size() -> (u16, u16) {
    let (cols, rows) = size().unwrap();

    (rows, cols)
}
