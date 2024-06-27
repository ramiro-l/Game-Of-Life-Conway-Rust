mod menu;
mod terminal;
mod ui;
pub use menu::Option;
pub use terminal::{clear_screen, cursor_hidden, terminal_size};
pub use ui::UserInterface;

pub const PADDING_W: u16 = 2;
pub const PADDING_H: u16 = 4;

pub const TITLE: &str = "\x1B[1mConway Game of Life\x1B[0m";
pub const DESCRPTION: &str = "Press 'KEY + Enter':";
