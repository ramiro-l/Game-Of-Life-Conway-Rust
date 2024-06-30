mod interaction;
mod menu;
mod menu_option;
pub mod terminal;
mod ui;

pub use interaction::Interaction;
pub use menu_option::{MenuOption, SpeedMenuOption};
pub use ui::UserInterface;

// Constants for UI
pub const PADDING_W: u16 = 2;
pub const PADDING_H: u16 = 4;
