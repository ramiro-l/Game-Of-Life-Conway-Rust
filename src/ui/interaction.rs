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
        let (interaction_tx, interaction_rx) = mpsc::channel();

        thread::spawn(move || loop {
            if let Some(option) = detect_interaction() {
                if interaction_tx.send(option).is_err() {
                    break;
                }
            }
        });

        Interaction { rx: interaction_rx }
    }
}

fn detect_interaction() -> Option<MenuOption> {
    if event::poll(Duration::from_millis(0)).unwrap_or(false) {
        match event::read().unwrap_or(Event::Resize(0, 0)) {
            Event::Mouse(mouse_event) if is_left_mouse_button_down(&mouse_event) => {
                return Some(MenuOption::from_mouse(mouse_event.column, mouse_event.row));
            }
            Event::Key(key_event) if is_ctrl_c_pressed(&key_event) => {
                return Some(MenuOption::Quit);
            }
            Event::Key(key_event) => {
                return Some(MenuOption::from_key(key_event.code));
            }
            _ => {}
        }
    }
    None
}

fn is_left_mouse_button_down(mouse_event: &crossterm::event::MouseEvent) -> bool {
    mouse_event.kind == crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left)
}

fn is_ctrl_c_pressed(key_event: &crossterm::event::KeyEvent) -> bool {
    key_event.modifiers.contains(KeyModifiers::CONTROL) && key_event.code == KeyCode::Char('c')
}
