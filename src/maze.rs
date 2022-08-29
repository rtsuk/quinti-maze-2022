use heapless::Vec;
use rand::{prelude::*, Rng, SeedableRng};
use rand_chacha::ChaChaRng;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum VisibleDoors {
    Left,
    Forward,
    Right,
    Up,
    Down,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Coord {
    pub fn move_in_direction(&self, direction: Direction) -> Self {
        let deltas = match direction {
            Direction::North => (0, -1, 0),
            Direction::South => (0, 1, 0),
            Direction::West => (-1, 0, 0),
            Direction::East => (1, 0, 0),
            Direction::Up => (0, 0, 1),
            Direction::Down => (0, 0, -1),
        };
        Self {
            x: self.x + deltas.0,
            y: self.y + deltas.1,
            z: self.z + deltas.2,
        }
    }
}

type Doors = [bool; 6];

#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    pub doors: Doors,
    pub visited: bool,
}

impl Cell {
    pub fn left(&self, _facing: Direction) -> bool {
        self.doors[Direction::West as usize]
    }

    pub fn front(&self, _facing: Direction) -> bool {
        self.doors[Direction::North as usize]
    }

    pub fn right(&self, _facing: Direction) -> bool {
        self.doors[Direction::East as usize]
    }

    pub fn top(&self) -> bool {
        self.doors[Direction::Up as usize]
    }

    pub fn bottom(&self) -> bool {
        self.doors[Direction::Down as usize]
    }

    pub fn remove_wall(&mut self, direction: &Direction) {
        self.doors[*direction as usize] = true;
    }
}

#[derive(Debug)]
pub struct Maze<const X: usize, const Y: usize, const Z: usize> {
    cells: [[[Cell; X]; Y]; Z],
}

impl<const X: usize, const Y: usize, const Z: usize> Default for Maze<X, Y, Z> {
    fn default() -> Self {
        Self {
            cells: [[[Cell::default(); X]; Y]; Z],
        }
    }
}

impl<const X: usize, const Y: usize, const Z: usize> Maze<X, Y, Z> {
    pub const fn dimensions() -> (usize, usize, usize) {
        (X, Y, Z)
    }

    const fn cell_count() -> usize {
        let dimensions = Self::dimensions();
        dimensions.0 * dimensions.1 * dimensions.2
    }

    fn validate_coord(&self, coord: &Coord) {
        let dimensions = Self::dimensions();
        assert!(0 <= coord.x);
        assert!(0 <= coord.y);
        assert!(0 <= coord.z);
        assert!((coord.x as usize) < dimensions.0);
        assert!((coord.y as usize) < dimensions.1);
        assert!((coord.z as usize) < dimensions.2);
    }

    pub fn get_cell(&self, coord: &Coord) -> Cell {
        self.validate_coord(coord);
        self.cells[coord.z as usize][coord.y as usize][coord.x as usize]
    }

    pub fn get_cell_mut(&mut self, coord: &Coord) -> &mut Cell {
        self.validate_coord(coord);
        &mut self.cells[coord.z as usize][coord.y as usize][coord.x as usize]
    }

    pub fn is_win(&self, coord: &Coord) -> bool {
        if coord.x < 0 || coord.y < 0 || coord.z < 0 {
            return true;
        }
        let dimensions = Self::dimensions();
        coord.x > dimensions.0 as isize
            || coord.y > dimensions.1 as isize
            || coord.z > dimensions.2 as isize
    }
}

type QuintiMaze = Maze<5, 5, 5>;
const CELL_COUNT: usize = QuintiMaze::cell_count();

#[derive(Default)]
pub struct MazeGenerator {
    maze: QuintiMaze,
    cells: Vec<Coord, CELL_COUNT>,
}

impl MazeGenerator {
    fn get_next_cell_coords(&mut self, coord: Coord, direction: &Direction) -> Option<Coord> {
        let dimensions = QuintiMaze::dimensions();
        let deltas = match *direction {
            Direction::North => (0, -1, 0),
            Direction::South => (0, 1, 0),
            Direction::West => (-1, 0, 0),
            Direction::East => (1, 0, 0),
            Direction::Up => (0, 0, 1),
            Direction::Down => (0, 0, -1),
        };
        let x = coord.x + deltas.0;
        if x < 0 || x >= dimensions.0 as isize {
            return None;
        }
        let y = coord.y + deltas.1;
        if y < 0 || y >= dimensions.1 as isize {
            return None;
        }
        let z = coord.z + deltas.2;
        if z < 0 || z >= dimensions.2 as isize {
            return None;
        }
        Some(Coord { x, y, z })
    }

    fn is_cell_visited(&self, coord: Coord) -> bool {
        self.maze.get_cell(&coord).visited
    }

    fn carve_passage(&mut self, coord: Coord, direction: &Direction) -> Option<Coord> {
        let next = self.get_next_cell_coords(coord, direction)?;
        let opposite = direction.opposite();
        let current = self.maze.get_cell_mut(&coord);
        current.visited = true;
        current.remove_wall(direction);
        let next_cell = self.maze.get_cell_mut(&next);
        next_cell.visited = true;
        next_cell.remove_wall(&opposite);

        Some(next)
    }

    pub fn generate(&mut self) {
        let mut rng = ChaChaRng::seed_from_u64(12);
        let (max_x, max_y, max_z) = QuintiMaze::dimensions();
        let x = rng.gen_range(0..max_x) as isize;
        let y = rng.gen_range(0..max_y) as isize;
        let z = rng.gen_range(0..max_z) as isize;

        let mut directions: Vec<Direction, 6> = Default::default();
        directions.extend([
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
            Direction::Up,
            Direction::Down,
        ]);

        self.cells.push(Coord { x, y, z }).expect("push");

        while !self.cells.is_empty() {
            let mut index = Some(rng.gen_range(0..self.cells.len()));
            let coords = self.cells[index.unwrap_or(0)];

            directions.shuffle(&mut rng);
            for dir in &directions {
                let next = match self.get_next_cell_coords(coords, dir) {
                    Some(next) => next,
                    None => continue,
                };

                if self.is_cell_visited(next) {
                    continue;
                }

                if let Some(next) = self.carve_passage(coords, dir) {
                    self.cells.push(next).expect("push");
                    index = None;
                    break;
                }
            }

            if let Some(index) = index {
                self.cells.remove(index);
            }
        }
    }

    pub fn take(self) -> QuintiMaze {
        self.maze
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate() {
        let mut generator = MazeGenerator::default();

        generator.generate();
    }
}
