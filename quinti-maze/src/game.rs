use crate::{
    draw::{
        draw_bottom_door, draw_front_door, draw_left_door, draw_right_door, draw_room, draw_status,
        draw_top_door, draw_win,
    },
    maze::{find_path_to_exit, Coord, Direction, QuintiMaze, VisibleDoors},
};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

pub struct Game {
    pub maze: QuintiMaze,
    position: Coord,
    needs_full_draw: bool,
    show_position: bool,
    direction_hint: Option<Direction>,
    facing: Direction,
}

impl Game {
    pub fn new(maze: QuintiMaze) -> Self {
        Self {
            maze,
            position: Coord::default(),
            needs_full_draw: true,
            show_position: false,
            direction_hint: None,
            facing: Direction::North,
        }
    }

    pub fn draw<D>(&mut self, display: &mut D, elapsed: u64) -> Result<(), D::Error>
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
            elapsed,
            needs_full_draw,
        )?;

        Ok(())
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
        }

        self.needs_full_draw = true;
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
        let _ = path.pop();
        let next_position = path.pop();
        if let Some(next_position) = next_position {
            self.direction_hint = Some(self.position.direction_to(next_position));
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
}
