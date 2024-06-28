mod interaction;
mod menu;
mod options;
pub mod terminal;
mod ui;

pub use interaction::Interaction;
pub use options::Option;
pub use options::SpeedOption;
pub use ui::UserInterface;

// Constants for UI
pub const PADDING_W: u16 = 2;
pub const PADDING_H: u16 = 4;
