mod ui;
mod world;

use crossterm::event::{self, Event};
use std::thread;
use std::time::Duration;

const DEFAULT_FRAME_RATE: u64 = 140;
const DEFAULT_POB: f32 = 0.5;

fn main() -> std::io::Result<()> {
    let (rows, cols) = ui::terminal_size();

    // Config UI
    let ui = ui::UserInterface::new(rows, cols);

    // Config World
    let mut world = world::World::new(rows, cols);
    world.random_map(DEFAULT_POB);

    // Config UI
    ui::cursor_hidden();

    // Start game
    'game: loop {
        ui::clear_screen();
        ui.draw_map(&world.map);
        ui.draw_border();
        ui.draw_menu();
        ui.print();
        world.next_iteration();

        match detect_key_press() {
            ui::Option::PauseAndResume => 'pause: loop {
                match detect_key_press() {
                    ui::Option::PauseAndResume => break 'pause,
                    ui::Option::Quit => break 'game,
                    _ => {}
                }
            },
            ui::Option::Quit => {
                break 'game;
            }
            ui::Option::New => {
                world.random_map(DEFAULT_POB);
            }
            ui::Option::Any => {}
        }

        thread::sleep(Duration::from_millis(DEFAULT_FRAME_RATE));
    }

    Ok(())
}

fn detect_key_press() -> ui::Option {
    if event::poll(Duration::from_millis(0)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            return ui::Option::from_key(key_event.code);
        }
    }
    ui::Option::Any
}
