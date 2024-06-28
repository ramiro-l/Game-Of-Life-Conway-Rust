use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Write};

pub fn config_init() {
    stdout().execute(EnterAlternateScreen).unwrap();
    stdout().execute(EnableMouseCapture).unwrap();
    enable_raw_mode().unwrap();
    cursor_hidden();
}

pub fn config_out() {
    stdout().execute(LeaveAlternateScreen).unwrap();
    stdout().execute(DisableMouseCapture).unwrap();
    disable_raw_mode().unwrap();
    cursor_show();
}

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

pub fn cursor_show() {
    print!("\x1B[?25h");
    print();
}

pub fn clear_screen() {
    print!("\x1B[2J");
    print();
}

pub fn size() -> (u16, u16) {
    let (cols, rows) = crossterm::terminal::size().unwrap();

    (rows, cols)
}
