mod ui;
mod world;

use crossterm::event::{self, Event};
use std::thread;
use std::time::Duration;
use world::WorldStatus;

const DEFAULT_FRAME_RATE: u64 = 140;
const DEFAULT_POB: f32 = 0.5;

fn main() -> std::io::Result<()> {
    let (rows, cols) = ui::terminal_size();

    // Config UI
    let ui = ui::UserInterface::new(rows, cols);
    ui::cursor_hidden();

    // Config World
    let mut world = world::World::new(rows, cols);
    world.random_map(DEFAULT_POB);

    // Start game
    'game: loop {
        ui::clear_screen();
        ui.draw_map(&world.map);
        ui.draw_border();
        ui.draw_menu();
        ui.print();
        world.next_iteration();

        let status: WorldStatus = match detect_key_press() {
            ui::Option::Quit => quit_game(),
            ui::Option::New => new_game(&mut world),
            ui::Option::PauseAndResume => pause_game(),
            ui::Option::Any => WorldStatus::Alive,
        };

        if status == WorldStatus::Dead {
            break 'game;
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

fn quit_game() -> WorldStatus {
    ui::clear_screen();
    WorldStatus::Dead
}

fn new_game(world: &mut world::World) -> WorldStatus {
    world.random_map(DEFAULT_POB);
    WorldStatus::Alive
}

fn pause_game() -> WorldStatus {
    'pause: loop {
        match detect_key_press() {
            ui::Option::PauseAndResume => break 'pause,
            ui::Option::Quit => {
                quit_game();
                return WorldStatus::Dead;
            }
            _ => {}
        }
    }
    WorldStatus::Alive
}
