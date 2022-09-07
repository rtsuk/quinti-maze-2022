#![no_std]
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettings, SimulatorDisplay, SimulatorEvent, Window,
};
use quinti_maze::{
    draw::SCREEN_SIZE,
    game::Game,
    maze::{MazeGenerator, VisibleDoors},
    time::Timer,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(SCREEN_SIZE);

    let mut generator = MazeGenerator::default();
    generator.generate(Some(13));
    let maze = generator.take();
    let timer = Timer::default();

    let mut game = Game::new(maze);

    let output_settings = OutputSettings::default();
    let mut window = Window::new("Quinti-Maze", &output_settings);

    loop {
        if game.is_win() {
            break;
        }

        game.draw(&mut display, timer.elapsed())?;

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => {
                    return Ok(());
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::W => {
                        game.try_move(VisibleDoors::Forward);
                    }
                    Keycode::D => {
                        game.try_move(VisibleDoors::Right);
                    }
                    Keycode::A => {
                        game.try_move(VisibleDoors::Left);
                    }
                    Keycode::E => {
                        game.try_move(VisibleDoors::Up);
                    }
                    Keycode::Q => {
                        game.try_move(VisibleDoors::Down);
                    }
                    Keycode::Left => {
                        game.turn_left();
                    }
                    Keycode::Right => {
                        game.turn_right();
                    }
                    Keycode::Slash => {
                        game.toggle_show_position();
                    }
                    Keycode::Equals => {
                        game.show_direction_hint();
                    }
                    _ => {}
                },
                _ => (),
            }
        }
    }

    game.draw_win(&mut display)?;
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
