use super::menu::Menu;
use super::terminal;
use super::{PADDING_H, PADDING_W};

const POINT: &str = "██";
const CHAR_LINE_W_B: &str = "▄";
const CHAR_LINE_W_T: &str = "▀";
const CHAR_LINE_H_R: &str = "▌";
const CHAR_LINE_H_L: &str = "▐";
const CHAR_LINE_H: &str = "█";

pub struct UserInterface {
    rows: u16,
    cols: u16,
    menu: Menu,
}

impl UserInterface {
    pub fn new(rows: u16, cols: u16) -> UserInterface {
        let menu = Menu::new();
        if cols <= menu.cols || rows <= menu.rows {
            super::terminal::config_out();
            println!("\nThe terminal is too small, please resize it.");
            std::process::exit(0);
        }

        UserInterface { rows, cols, menu }
    }

    pub fn draw_point(&self, row: u16, mut col: u16) {
        // Check if the point is inside the menu
        if row < self.menu.rows && col < self.menu.cols {
            return;
        }

        if col % 2 == 1 {
            col = col + 1;
        };
        terminal::draw_on(row + PADDING_W, col + PADDING_H, POINT);
    }

    pub fn draw_border(&self) {
        for i in 0..self.cols {
            terminal::draw_on(0, i, CHAR_LINE_W_T);
            terminal::draw_on(self.rows, i, CHAR_LINE_W_B);
        }

        for i in 0..self.rows + 1 {
            terminal::draw_on(i, 0, CHAR_LINE_H_L);
            terminal::draw_on(i, self.cols, CHAR_LINE_H_R);
        }
    }

    pub fn draw_menu(&self) {
        // Add padding
        let x = 3;
        let y = 2;

        // Draw texts
        for i in 0..self.menu.texts.len() {
            terminal::draw_on(y + (i as u16), x, self.menu.texts[i as usize]);
        }

        // Draw borders
        for i in 2..self.menu.cols {
            terminal::draw_on(self.menu.rows, i, CHAR_LINE_W_B);
        }
        for i in 1..self.menu.rows + 1 {
            terminal::draw_on(i, self.menu.cols, CHAR_LINE_H);
        }
    }

    pub fn draw_map(&self, map: &crate::world::WorldMap) {
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

    pub fn print(&self) {
        terminal::print();
    }
}
