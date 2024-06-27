mod cell;
mod ui;
mod world;

use crossterm::event::{self, Event, KeyCode};
use std::thread;
use std::time::Duration;
use ui::UserInterface;
use world::World;

const FRAME_RATE: u64 = 140;
const DEFAULT_POB: f32 = 0.5;

fn main() -> std::io::Result<()> {
    let ui = UserInterface::new();

    // Config World
    let (rows, cols) = ui.get_terminal_size();
    let mut world = World::new(rows, cols);
    world.random_map(DEFAULT_POB);

    // Config UI
    ui.cursor_hidden();

    // Start game
    loop {
        ui.clear_screen();
        ui.draw_map(&world.map);
        ui.draw_border();
        ui.draw_menu();
        ui.print();
        world.next_iteration();

        // Detect key presses
        if let Some(key) = detect_key_press() {
            match key {
                KeyCode::Char('q') => {
                    ui.clear_screen();
                    return Ok(());
                }
                KeyCode::Char('r') => {
                    world.random_map(DEFAULT_POB);
                }
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(FRAME_RATE));
    }
}

fn detect_key_press() -> Option<KeyCode> {
    if event::poll(Duration::from_millis(0)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            return Some(key_event.code);
        }
    }
    None
}
