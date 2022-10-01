use crate::{
    draw::{
        draw_bottom_door, draw_front_door, draw_left_door, draw_right_door, draw_room, draw_start,
        draw_status, draw_top_door, draw_win,
    },
    maze::{
        find_path_to_exit, Coord, Direction, MazeGenerator, QuintiMaze, SolutionPath, VisibleDoors,
    },
};
use core::fmt::Debug;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

pub const NOTES: &[(u32, u64, u64)] = &[
    (1000, 256, 0),
    (1000, 128, 50),
    (1000, 128, 50),
    (1333, 169, 50),
    (1000, 169, 50),
    (1333, 169, 50),
    (1667, 653, 50),
];

pub trait PlatformSpecific: Debug + Default {
    fn play_victory_notes(&mut self);
    fn ticks(&mut self) -> u64;
}

enum Phase {
    Start,
    Playing,
    Done,
}

pub enum Command {
    MoveForward,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    TurnLeft,
    TurnRight,
    ToggleShowPosition,
    ShowHints,
}

pub struct Game<T: PlatformSpecific> {
    pub maze: QuintiMaze,
    platform: T,
    phase: Phase,
    position: Coord,
    needs_full_draw: bool,
    show_position: bool,
    direction_hint: Option<Direction>,
    path_to_exit: Option<SolutionPath>,
    facing: Direction,
    start: u64,
}

impl<T: PlatformSpecific> Default for Game<T> {
    fn default() -> Self {
        let mut platform = T::default();
        let mut generator = MazeGenerator::default();
        generator.generate(Some(platform.ticks()));
        let maze = generator.take();
        Self {
            maze,
            platform: Default::default(),
            phase: Phase::Start,
            position: Coord::default(),
            needs_full_draw: true,
            show_position: false,
            direction_hint: None,
            path_to_exit: None,
            facing: Direction::North,
            start: platform.ticks(),
        }
    }
}

impl<T: PlatformSpecific> Game<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn make_new_maze(&mut self) {
        let mut generator = MazeGenerator::default();
        generator.generate(Some(self.platform.ticks()));
        self.maze = generator.take();
        self.position = Coord::default();
        self.start = self.platform.ticks();
    }

    pub fn draw_playing<D>(&mut self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let needs_full_draw = self.needs_full_draw;
        if needs_full_draw {
            display.clear(Rgb565::WHITE)?;

            let cell = self.maze.get_cell(&self.position);

            draw_room(display)?;
            if cell.right(self.facing) {
                draw_right_door(display)?;
            }

            if cell.left(self.facing) {
                draw_left_door(display)?;
            }

            if cell.top() {
                draw_top_door(display)?;
            }

            if cell.bottom() {
                draw_bottom_door(display)?;
            }

            if cell.front(self.facing) {
                draw_front_door(display)?;
            }

            self.needs_full_draw = false;
        }

        draw_status(
            display,
            self.facing,
            self.show_position.then_some(self.position),
            self.direction_hint,
            self.platform.ticks() - self.start,
            needs_full_draw,
        )?;

        Ok(())
    }

    pub fn draw<D>(&mut self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        match self.phase {
            Phase::Playing => self.draw_playing(display)?,
            Phase::Done => self.draw_win(display)?,
            Phase::Start => self.draw_start(display)?,
        }

        Ok(())
    }

    pub fn key_hit(&mut self) -> bool {
        match self.phase {
            Phase::Playing => true,
            Phase::Start => {
                self.phase = Phase::Playing;
                self.start = self.platform.ticks();
                false
            }
            Phase::Done => {
                self.position = Coord::default();
                self.phase = Phase::Start;
                false
            }
        }
    }

    pub fn try_move(&mut self, door: VisibleDoors) {
        let cell = self.maze.get_cell(&self.position);

        let old_position = self.position;

        let direction = door.direction(self.facing);

        if cell.has_door(direction) {
            self.position = self.position.move_in_direction(direction);
        }

        if self.position != old_position {
            self.direction_hint = None;
            let path_to_exit = self.path_to_exit.take();
            if let Some(mut path_to_exit) = path_to_exit {
                let on_path = path_to_exit.pop_back();
                if on_path == Some(self.position) {
                    if let Some(next_position) = path_to_exit.back() {
                        self.direction_hint = Some(self.position.direction_to(*next_position));
                        self.path_to_exit = Some(path_to_exit);
                    }
                }
            }
        }

        self.needs_full_draw = true;

        if self.is_win() {
            self.phase = Phase::Done;
            self.platform.play_victory_notes();
            self.make_new_maze();
        }
    }

    pub fn turn_left(&mut self) {
        self.facing = VisibleDoors::Left.direction(self.facing);
        self.needs_full_draw = true;
    }

    pub fn turn_right(&mut self) {
        self.facing = VisibleDoors::Right.direction(self.facing);
        self.needs_full_draw = true;
    }

    pub fn toggle_show_position(&mut self) {
        self.show_position = !self.show_position;
    }

    pub fn show_direction_hint(&mut self) {
        let (_found, mut path) = find_path_to_exit(&self.maze, self.position);
        path.pop_back();
        let next_position = path.back();
        if let Some(next_position) = next_position {
            self.direction_hint = Some(self.position.direction_to(*next_position));
            self.path_to_exit = Some(path);
        }
    }

    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::MoveForward => {
                self.try_move(VisibleDoors::Forward);
            }
            Command::MoveRight => {
                self.try_move(VisibleDoors::Right);
            }
            Command::MoveLeft => {
                self.try_move(VisibleDoors::Left);
            }
            Command::MoveUp => {
                self.try_move(VisibleDoors::Up);
            }
            Command::MoveDown => {
                self.try_move(VisibleDoors::Down);
            }
            Command::TurnLeft => {
                self.turn_left();
            }
            Command::TurnRight => {
                self.turn_right();
            }
            Command::ToggleShowPosition => {
                self.toggle_show_position();
            }
            Command::ShowHints => {
                self.show_direction_hint();
            }
        }
    }

    pub fn is_win(&self) -> bool {
        self.maze.is_win(&self.position)
    }

    pub fn draw_win<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        display.clear(Rgb565::BLACK)?;
        draw_win(display)?;
        Ok(())
    }

    pub fn draw_start<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        display.clear(Rgb565::BLACK)?;
        draw_start(display)?;
        Ok(())
    }
}
