#![no_std]
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettings, SimulatorDisplay, SimulatorEvent, Window,
};

mod draw;
mod maze;

use crate::{
    draw::{
        draw_bottom_door, draw_front_door, draw_left_door, draw_right_door, draw_room, draw_status,
        draw_top_door, SCREEN_SIZE,
    },
    maze::{Coord, Direction, MazeGenerator, VisibleDoors},
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(SCREEN_SIZE);

    let mut generator = MazeGenerator::default();
    generator.generate();
    let maze = generator.take();

    let mut position = Coord { x: 3, y: 3, z: 3 };
    let mut facing = Direction::North;
    let mut show_position = false;

    let output_settings = OutputSettings::default();
    let mut window = Window::new("Quinti-Maze", &output_settings);

    'running: loop {
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

        draw_status(&mut display, facing, show_position.then_some(position))?;

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => {
                    break 'running;
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::W => {
                        if cell.front(facing) {
                            position =
                                position.move_in_direction(VisibleDoors::Forward.direction(facing));
                        }
                    }
                    Keycode::D => {
                        if cell.right(facing) {
                            position =
                                position.move_in_direction(VisibleDoors::Right.direction(facing));
                        }
                    }
                    Keycode::A => {
                        if cell.left(facing) {
                            position =
                                position.move_in_direction(VisibleDoors::Left.direction(facing));
                        }
                    }
                    Keycode::E => {
                        if cell.top() {
                            position = position.move_in_direction(Direction::Up);
                        }
                    }
                    Keycode::Q => {
                        if cell.bottom() {
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
                    _ => {
                        //                        dbg!(event);
                    }
                },
                _ => (),
            }
        }
    }

    Ok(())
}
