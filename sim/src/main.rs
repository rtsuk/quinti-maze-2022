#![no_std]
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettings, SimulatorDisplay, SimulatorEvent, Window,
};
use quinti_maze::{
    draw::SCREEN_SIZE,
    game::{Command, Game},
    maze::MazeGenerator,
    time::Timer,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(SCREEN_SIZE);

    let mut generator = MazeGenerator::default();
    generator.generate(Some(13));
    let maze = generator.take();

    let mut game = Game::new(maze);
    let timer = Timer::default();

    let output_settings = OutputSettings::default();
    let mut window = Window::new("Quinti-Maze", &output_settings);

    loop {
        game.draw(&mut display, timer.elapsed())?;

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => {
                    return Ok(());
                }
                SimulatorEvent::KeyDown { keycode, .. } => {
                    if game.key_hit() {
                        match keycode {
                            Keycode::W => {
                                game.handle_command(Command::MoveForward);
                            }
                            Keycode::D => {
                                game.handle_command(Command::MoveRight);
                            }
                            Keycode::A => {
                                game.handle_command(Command::MoveLeft);
                            }
                            Keycode::E => {
                                game.handle_command(Command::MoveUp);
                            }
                            Keycode::Q => {
                                game.handle_command(Command::MoveDown);
                            }
                            Keycode::Left => {
                                game.handle_command(Command::TurnLeft);
                            }
                            Keycode::Right => {
                                game.handle_command(Command::TurnRight);
                            }
                            Keycode::Slash => {
                                game.handle_command(Command::ToggleShowPosition);
                            }
                            Keycode::Equals => {
                                game.handle_command(Command::ShowHints);
                            }
                            _ => {}
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
