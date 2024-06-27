use crossterm::terminal::size;
use std::io::Write;

fn move_cursor(row: u16, col: u16) {
    print!("\x1B[{};{}H", row, col);
}

pub fn print() {
    std::io::stdout().flush().unwrap();
}

pub fn draw_on(row: u16, col: u16, c: &str) {
    move_cursor(row, col);
    print!("{}", c);
}

pub fn cursor_hidden() {
    print!("\x1B[?25l");
    print();
}

pub fn clear_screen() {
    print!("\x1B[2J");
    print();
}

pub fn terminal_size() -> (u16, u16) {
    let (cols, rows) = size().unwrap();

    (rows, cols)
}
