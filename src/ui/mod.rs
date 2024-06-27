mod menu;
mod options;
mod terminal;
mod ui;

pub use options::Option;
pub use terminal::{clear_screen, cursor_hidden, terminal_size};
pub use ui::UserInterface;

// Constants for UI
pub const PADDING_W: u16 = 2;
pub const PADDING_H: u16 = 4;
