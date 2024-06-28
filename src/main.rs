mod ui;
mod world;

use std::{io::Result, thread, time::Duration};
use world::World;

const DEFAULT_FRAME_RATE: u64 = 140;
const DEFAULT_POB: f32 = 0.5;

fn main() -> Result<()> {
    // Config terminal
    ui::terminal::config_init();
    let (rows, cols) = ui::terminal::size();

    // Config UI
    let ui = ui::UserInterface::new(rows, cols);
    ui::terminal::cursor_hidden();

    // Config World
    let mut world = World::new(rows, cols);
    world.random_map(DEFAULT_POB);

    // Config Interaction
    let interaction = ui::Interaction::new();

    // Start Game
    while !world.is_dead() {
        ui::terminal::clear_screen();
        ui.draw_map(&world.map);
        ui.draw_border();
        ui.draw_menu();
        ui.print();

        while let Ok(option) = interaction.rx.try_recv() {
            handle_interaction(&mut world, option);
        }

        if !world.is_paused() {
            world.next_iteration();
        }

        thread::sleep(Duration::from_millis(DEFAULT_FRAME_RATE));
    }

    ui::terminal::config_out();
    Ok(())
}

fn handle_interaction(world: &mut World, option: ui::Option) {
    match option {
        ui::Option::PauseAndResume => world.toggle_pause(),
        ui::Option::NewRandomMap => world.random_map(DEFAULT_POB),
        ui::Option::Quit => world.kill(),
        ui::Option::Edit(row, col) => world.toggle_cell(row, col),
        ui::Option::Clear => world.clear(),
        ui::Option::Any => {}
    };
}
