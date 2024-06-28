mod ui;
mod world;

use std::{io::Result, thread, time::Duration};
use world::World;

const DEFAULT_FRAME_RATE: u64 = 140;
const STEP_FRAME_RATE: u64 = 10;
const DEFAULT_RANDOM_POB: f32 = 0.5;

fn main() -> Result<()> {
    // Config terminal
    std::panic::set_hook(Box::new(|_| {
        ui::terminal::config_out();
    }));
    ui::terminal::config_init();
    let (rows, cols) = ui::terminal::size();

    // Config UI
    let ui = ui::UserInterface::new(rows, cols);
    let mut frame_rate = DEFAULT_FRAME_RATE;

    // Config World
    let mut world = World::new(rows, cols);
    world.random_map(DEFAULT_RANDOM_POB);

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
            handle_interaction(&mut world, option, &mut frame_rate);
        }

        if !world.is_paused() {
            world.next_iteration();
        }

        thread::sleep(Duration::from_millis(frame_rate));
    }

    ui::terminal::config_out();
    Ok(())
}

fn handle_interaction(world: &mut World, option: ui::Option, frame_rate: &mut u64) {
    match option {
        ui::Option::PauseAndResume => world.toggle_pause(),
        ui::Option::NewRandomMap => world.random_map(DEFAULT_RANDOM_POB),
        ui::Option::Quit => world.kill(),
        ui::Option::Edit(row, col) => world.toggle_cell(row, col),
        ui::Option::Clear => world.clear(),
        ui::Option::Speed(speed_option) => handle_speed(speed_option, frame_rate),
        ui::Option::Any => {}
    };
}

fn handle_speed(speed_option: ui::SpeedOption, frame_rate: &mut u64) {
    match speed_option {
        ui::SpeedOption::Less => *frame_rate += STEP_FRAME_RATE,
        ui::SpeedOption::More => {
            if *frame_rate > STEP_FRAME_RATE {
                *frame_rate -= STEP_FRAME_RATE;
            }
        }
    }
}
