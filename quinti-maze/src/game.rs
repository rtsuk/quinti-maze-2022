use crate::{
    draw::{
        draw_bottom_door, draw_front_door, draw_left_door, draw_right_door, draw_room, draw_start,
        draw_status, draw_top_door, draw_win, update_time,
    },
    maze::{
        find_path_to_exit, Coord, Direction, MazeGenerator, QuintiMaze, SolutionPath, VisibleDoors,
    },
};
use core::fmt::Debug;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

pub struct Note {
    pub duration: u32,
    pub frequency: u64,
    pub delay: u64,
}

impl Note {
    pub const fn new(frequency: u64, duration: u32, delay: u64) -> Self {
        Self {
            duration,
            frequency,
            delay,
        }
    }
}

pub const NOTES: &[Note] = &[
    Note::new(1000, 256, 0),
    Note::new(1000, 128, 50),
    Note::new(1000, 128, 50),
    Note::new(1333, 169, 50),
    Note::new(1000, 169, 50),
    Note::new(1333, 169, 50),
    Note::new(1667, 653, 50),
];

pub trait PlatformSpecific: Debug + Default {
    fn play_victory_notes(&mut self);
    fn ticks(&mut self) -> u64;
}

#[derive(Debug, PartialEq)]
enum RedrawMode {
    Time,
    Status,
    Full,
}

impl Default for RedrawMode {
    fn default() -> Self {
        Self::Full
    }
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

#[derive(Debug, Default)]
struct Showing {
    pub left: bool,
    pub front: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

#[derive(Debug, Default)]
struct PlayingPhaseData {
    maze: QuintiMaze,
    position: Coord,
    next_redraw: RedrawMode,
    show_position: bool,
    direction_hint: Option<Direction>,
    path_to_exit: Option<SolutionPath>,
    facing: Direction,
    start: u64,
    showing: Showing,
}

impl PlayingPhaseData {
    pub fn new(ticks: u64) -> Self {
        let mut generator = MazeGenerator::default();
        generator.generate(Some(ticks));
        Self {
            maze: generator.take(),
            start: ticks,
            ..Default::default()
        }
    }

    pub fn draw_playing<D>(&mut self, ticks: u64, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        if self.next_redraw == RedrawMode::Full {
            self.showing = Default::default();
            display.clear(Rgb565::WHITE)?;
            draw_room(display)?;
        }

        let cell = self.maze.get_cell(&self.position);

        let showing_right = cell.right(self.facing);
        if showing_right != self.showing.right {
            draw_right_door(display, showing_right)?;
            self.showing.right = showing_right;
        }

        let showing_left = cell.left(self.facing);
        if showing_left != self.showing.left {
            draw_left_door(display, showing_left)?;
            self.showing.left = showing_left;
        }

        let showing_top = cell.top();
        if showing_top != self.showing.top {
            draw_top_door(display, showing_top)?;
            self.showing.top = showing_top;
        }

        let showing_bottom = cell.bottom();
        if showing_bottom != self.showing.bottom {
            draw_bottom_door(display, showing_bottom)?;
            self.showing.bottom = showing_bottom;
        }

        let showing_front = cell.front(self.facing);
        if showing_front != self.showing.front {
            draw_front_door(display, showing_front)?;
            self.showing.front = showing_front;
        }

        let elapsed = ticks - self.start;

        if self.next_redraw != RedrawMode::Time {
            draw_status(
                display,
                self.facing,
                self.show_position.then_some(self.position),
                self.direction_hint,
                elapsed,
            )?;
        } else {
            update_time(display, elapsed)?;
        }
        self.next_redraw = RedrawMode::Time;

        Ok(())
    }

    pub fn try_move(&mut self, door: VisibleDoors) -> bool {
        let cell = self.maze.get_cell(&self.position);

        let old_position = self.position;

        let direction = door.direction(self.facing);

        if cell.has_door(direction) {
            self.position = self.position.move_in_direction(direction);
        }

        if self.position != old_position {
            self.next_redraw = RedrawMode::Status;
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

        self.is_win()
    }

    pub fn turn_left(&mut self) {
        self.facing = VisibleDoors::Left.direction(self.facing);
        self.next_redraw = RedrawMode::Status;
    }

    pub fn turn_right(&mut self) {
        self.facing = VisibleDoors::Right.direction(self.facing);
        self.next_redraw = RedrawMode::Status;
    }

    pub fn toggle_show_position(&mut self) {
        self.show_position = !self.show_position;
        self.next_redraw = RedrawMode::Status;
    }

    pub fn show_direction_hint(&mut self) {
        let (_found, mut path) = find_path_to_exit(&self.maze, self.position);
        path.pop_back();
        let next_position = path.back();
        if let Some(next_position) = next_position {
            self.direction_hint = Some(self.position.direction_to(*next_position));
            self.path_to_exit = Some(path);
        }
        self.next_redraw = RedrawMode::Status;
    }

    pub fn handle_command(&mut self, command: Command) -> bool {
        let mut is_win = false;
        match command {
            Command::MoveForward => {
                is_win = self.try_move(VisibleDoors::Forward);
            }
            Command::MoveRight => {
                is_win = self.try_move(VisibleDoors::Right);
            }
            Command::MoveLeft => {
                is_win = self.try_move(VisibleDoors::Left);
            }
            Command::MoveUp => {
                is_win = self.try_move(VisibleDoors::Up);
            }
            Command::MoveDown => {
                is_win = self.try_move(VisibleDoors::Down);
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

        is_win
    }

    pub fn is_win(&self) -> bool {
        self.maze.is_win(&self.position)
    }
}

enum Phase {
    Start(bool),
    Playing(PlayingPhaseData),
    Done(bool),
}

pub struct Game<T: PlatformSpecific> {
    platform: T,
    phase: Phase,
}

impl<T: PlatformSpecific> Default for Game<T> {
    fn default() -> Self {
        Self {
            platform: Default::default(),
            phase: Phase::Start(false),
        }
    }
}

impl<T: PlatformSpecific> Game<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn draw_start<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        display.clear(Rgb565::BLACK)?;
        draw_start(display)?;
        Ok(())
    }

    pub fn draw<D>(&mut self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        match &mut self.phase {
            Phase::Playing(playing_state) => {
                playing_state.draw_playing(self.platform.ticks(), display)?;
            }
            Phase::Done(drawn) => {
                if !*drawn {
                    *drawn = true;
                    self.draw_win(display)?;
                    self.platform.play_victory_notes();
                }
            }
            Phase::Start(drawn) => {
                if !*drawn {
                    *drawn = true;
                    self.draw_start(display)?;
                }
            }
        }

        Ok(())
    }

    pub fn draw_win<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        display.clear(Rgb565::BLACK)?;
        draw_win(display)?;
        Ok(())
    }

    pub fn key_hit(&mut self) -> bool {
        match self.phase {
            Phase::Playing(_) => true,
            Phase::Start(_) => {
                self.phase = Phase::Playing(PlayingPhaseData::new(self.platform.ticks()));
                false
            }
            Phase::Done(_) => {
                self.phase = Phase::Start(false);
                false
            }
        }
    }

    pub fn handle_command(&mut self, command: Command) {
        if let Phase::Playing(playing_state) = &mut self.phase {
            if playing_state.handle_command(command) {
                self.phase = Phase::Done(false);
            }
        }
    }
}
