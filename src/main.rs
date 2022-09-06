#![no_std]
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettings, SimulatorDisplay, SimulatorEvent, Window,
};

mod draw;
mod maze;
mod time;

use crate::{
    draw::{
        draw_bottom_door, draw_front_door, draw_left_door, draw_right_door, draw_room, draw_status,
        draw_top_door, draw_win, SCREEN_SIZE,
    },
    maze::{find_path_to_exit, Coord, Direction, MazeGenerator, VisibleDoors},
    time::Timer,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(SCREEN_SIZE);

    let mut generator = MazeGenerator::default();
    generator.generate(Some(13));
    let maze = generator.take();
    let timer = Timer::default();

    let mut position = Coord { x: 0, y: 0, z: 0 };
    let mut facing = Direction::North;
    let mut show_position = false;
    let mut direction_hint = None;

    let output_settings = OutputSettings::default();
    let mut window = Window::new("Quinti-Maze", &output_settings);

    loop {
        display.clear(Rgb565::WHITE)?;

        if maze.is_win(&position) {
            break;
        }

        let cell = maze.get_cell(&position);

        draw_room(&mut display)?;
        if cell.right(facing) {
            draw_right_door(&mut display)?;
        }
        if cell.left(facing) {
            draw_left_door(&mut display)?;
        }
        if cell.top() {
            draw_top_door(&mut display)?;
        }
        if cell.bottom() {
            draw_bottom_door(&mut display)?;
        }
        if cell.front(facing) {
            draw_front_door(&mut display)?;
        }

        draw_status(
            &mut display,
            facing,
            show_position.then_some(position),
            direction_hint,
            timer.elapsed(),
        )?;

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => {
                    return Ok(());
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::W => {
                        if cell.front(facing) {
                            direction_hint = None;
                            position =
                                position.move_in_direction(VisibleDoors::Forward.direction(facing));
                        }
                    }
                    Keycode::D => {
                        if cell.right(facing) {
                            direction_hint = None;
                            position =
                                position.move_in_direction(VisibleDoors::Right.direction(facing));
                        }
                    }
                    Keycode::A => {
                        if cell.left(facing) {
                            direction_hint = None;
                            position =
                                position.move_in_direction(VisibleDoors::Left.direction(facing));
                        }
                    }
                    Keycode::E => {
                        if cell.top() {
                            direction_hint = None;
                            position = position.move_in_direction(Direction::Up);
                        }
                    }
                    Keycode::Q => {
                        if cell.bottom() {
                            direction_hint = None;
                            position = position.move_in_direction(Direction::Down);
                        }
                    }
                    Keycode::Left => {
                        facing = VisibleDoors::Left.direction(facing);
                    }
                    Keycode::Right => {
                        facing = VisibleDoors::Right.direction(facing);
                    }
                    Keycode::Slash => {
                        show_position = !show_position;
                    }
                    Keycode::Equals => {
                        let (_found, mut path) = find_path_to_exit(&maze, position);
                        let _ = path.pop();
                        let next_position = path.pop();
                        if let Some(next_position) = next_position {
                            direction_hint = Some(position.direction_to(next_position));
                        }
                    }
                    _ => {}
                },
                _ => (),
            }
        }
    }

    display.clear(Rgb565::BLACK)?;
    draw_win(&mut display)?;
    window.update(&display);

    'win: loop {
        for event in window.events() {
            if event == SimulatorEvent::Quit {
                break 'win;
            }
        }
    }

    Ok(())
}
