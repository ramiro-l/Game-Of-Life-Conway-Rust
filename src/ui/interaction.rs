use super::MenuOption;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

pub struct Interaction {
    pub rx: Receiver<MenuOption>,
}

impl Interaction {
    pub fn new() -> Interaction {
        // Config Interaction
        let (interaction_tx, interaction_rx) = mpsc::channel();

        thread::spawn(move || loop {
            if let Some(option) = detect_interaction() {
                if option == MenuOption::Any {
                    continue;
                }
                if interaction_tx.send(option).is_err() {
                    break;
                }
            }
        });

        Interaction { rx: interaction_rx }
    }
}

fn detect_interaction() -> Option<MenuOption> {
    if event::poll(Duration::from_millis(0)).unwrap() {
        match event::read().unwrap() {
            Event::Mouse(mouse_event) => {
                if mouse_event.kind
                    == crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left)
                {
                    return Some(MenuOption::from_mouse(mouse_event.column, mouse_event.row));
                }
            }
            Event::Key(key_event) => {
                // Exit with 'Ctrl + c'
                if key_event.modifiers.contains(KeyModifiers::CONTROL)
                    && key_event.code == KeyCode::Char('c')
                {
                    return Some(MenuOption::Quit);
                }

                // Other key events
                return Some(MenuOption::from_key(key_event.code));
            }
            _ => {}
        }
    }
    None
}
