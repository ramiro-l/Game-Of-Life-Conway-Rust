mod ui;
mod world;

use crossterm::event::{self, Event};
use std::thread;
use std::time::Duration;
use world::World;

const DEFAULT_FRAME_RATE: u64 = 140;
const DEFAULT_POB: f32 = 0.5;

fn main() -> std::io::Result<()> {
    let (rows, cols) = ui::terminal_size();

    // Config UI
    let ui = ui::UserInterface::new(rows, cols);
    ui::cursor_hidden();

    // Config World
    let mut world = World::new(rows, cols);
    world.random_map(DEFAULT_POB);

    // Start World
    while !world.is_dead() {
        ui::clear_screen();
        ui.draw_map(&world.map);
        ui.draw_border();
        ui.draw_menu();
        ui.print();

        if !world.is_paused() {
            world.next_iteration();
        }

        handle_key_press(&mut world);

        thread::sleep(Duration::from_millis(DEFAULT_FRAME_RATE));
    }

    ui::clear_screen();
    Ok(())
}

fn handle_key_press(world: &mut World) {
    match detect_key_press() {
        ui::Option::PauseAndResume => world.toggel_pause(),
        ui::Option::NewRandomMap => world.random_map(DEFAULT_POB),
        ui::Option::Quit => world.kill(),
        ui::Option::Any => {}
    };
}

fn detect_key_press() -> ui::Option {
    if event::poll(Duration::from_millis(0)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            return ui::Option::from_key(key_event.code);
        }
    }
    ui::Option::Any
}
