#![no_std]
use core::{
    fmt::{Debug, Error, Formatter},
    time::Duration,
};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettings, SimulatorDisplay, SimulatorEvent, Window,
};
use quinti_maze::{
    draw::SCREEN_SIZE,
    game::{Command, Game, PlatformSpecific, NOTES},
    time::Timer,
};
use rodio::{source::SineWave, OutputStream, OutputStreamHandle, Sink, Source};

struct SimPlatform {
    timer: Timer,
    #[allow(unused)]
    stream: OutputStream,
    #[allow(unused)]
    stream_handle: OutputStreamHandle,
}

impl Default for SimPlatform {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().expect("default sound output");
        Self {
            timer: Timer::default(),
            stream,
            stream_handle,
        }
    }
}

impl Debug for SimPlatform {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Sprite")
            .field("timer", &self.timer)
            .finish()
    }
}

impl PlatformSpecific for SimPlatform {
    fn play_victory_notes(&mut self) {
        let sink = Sink::try_new(&self.stream_handle).expect("new sink");
        for (freq, duration, delay) in NOTES {
            let source = SineWave::new(*freq)
                .take_duration(Duration::from_millis(*duration))
                .amplify(0.20)
                .delay(Duration::from_millis(*delay));
            sink.append(source);
        }
        sink.sleep_until_end();
    }

    fn ticks(&mut self) -> u64 {
        self.timer.elapsed()
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(SCREEN_SIZE);

    let mut game = Game::<SimPlatform>::new();

    let output_settings = OutputSettings::default();
    let mut window = Window::new("Quinti-Maze", &output_settings);

    loop {
        game.draw(&mut display)?;

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
