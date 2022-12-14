use heapless::{Deque, Vec};
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

impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

impl From<Direction> for &'static str {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
            Direction::Up => "Up",
            Direction::Down => "Down",
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

impl VisibleDoors {
    pub fn direction(&self, facing: Direction) -> Direction {
        match self {
            Self::Up => Direction::Up,
            Self::Down => Direction::Down,
            Self::Left => match facing {
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                _ => Direction::West,
            },
            Self::Right => match facing {
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                _ => Direction::East,
            },
            Self::Forward => facing,
        }
    }

    pub fn direction_as_index(&self, facing: Direction) -> usize {
        self.direction(facing) as usize
    }
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

    pub fn direction_to(&self, target: Self) -> Direction {
        let delta_x = self.x - target.x;
        if delta_x != 0 {
            return if delta_x < 0 {
                Direction::East
            } else {
                Direction::West
            };
        }
        let delta_y = self.y - target.y;
        if delta_y != 0 {
            return if delta_y < 0 {
                Direction::South
            } else {
                Direction::North
            };
        }
        let delta_z = self.z - target.z;
        if delta_z != 0 {
            return if delta_z < 0 {
                Direction::Up
            } else {
                Direction::Down
            };
        }
        panic!("impossible direction");
    }
}

type Doors = [bool; 6];

#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    pub doors: Doors,
    pub visited: bool,
}

impl Cell {
    pub fn has_door(&self, direction: Direction) -> bool {
        self.doors[direction as usize]
    }

    pub fn left(&self, facing: Direction) -> bool {
        self.doors[VisibleDoors::Left.direction_as_index(facing)]
    }

    pub fn front(&self, facing: Direction) -> bool {
        self.doors[VisibleDoors::Forward.direction_as_index(facing)]
    }

    pub fn right(&self, facing: Direction) -> bool {
        self.doors[VisibleDoors::Right.direction_as_index(facing)]
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
        coord.x >= dimensions.0 as isize
            || coord.y >= dimensions.1 as isize
            || coord.z >= dimensions.2 as isize
    }
}

pub type QuintiMaze = Maze<5, 5, 5>;
pub const CELL_COUNT: usize = QuintiMaze::cell_count();

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

    pub fn generate(&mut self, seed: Option<u64>) {
        let mut rng = ChaChaRng::seed_from_u64(seed.unwrap_or(12));
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
        self.maze.cells[max_x - 1][max_z - 1][max_z - 1].remove_wall(&Direction::Up);
    }

    pub fn take(self) -> QuintiMaze {
        self.maze
    }
}

pub type SolutionPath = Deque<Coord, CELL_COUNT>;

fn find_exit(
    maze: &QuintiMaze,
    prior_location: Option<Coord>,
    location: Coord,
    result: &mut SolutionPath,
) -> bool {
    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::West,
        Direction::East,
        Direction::South,
        Direction::North,
    ] {
        let cell = maze.get_cell(&location);
        if cell.has_door(direction) {
            let new_location = location.move_in_direction(direction);
            if maze.is_win(&new_location) {
                result.push_back(new_location).expect("push");
                result.push_back(location).expect("push");
                return true;
            }
            if Some(new_location) != prior_location
                && find_exit(maze, Some(location), new_location, result)
            {
                result.push_back(location).expect("push");
                return true;
            }
        }
    }
    false
}

pub fn find_path_to_exit(maze: &QuintiMaze, starting_position: Coord) -> (bool, SolutionPath) {
    let mut vec = SolutionPath::new();

    let result = find_exit(maze, None, starting_position, &mut vec);

    (result, vec)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate() {
        let mut generator = MazeGenerator::default();

        generator.generate(None);
    }

    #[test]
    fn test_solve() {
        let mut generator = MazeGenerator::default();

        generator.generate(None);

        let maze = generator.take();

        let (found, mut path) = find_path_to_exit(&maze, Coord::default());

        assert!(found);
        assert_eq!(path.len(), 14);
        assert_eq!(path.pop_back(), Some(Coord { x: 0, y: 0, z: 0 }));
        assert_eq!(path.front().unwrap(), &Coord { x: 4, y: 4, z: 5 });

        let mut generator = MazeGenerator::default();

        generator.generate(Some(13));

        let maze = generator.take();

        let (found, mut path) = find_path_to_exit(&maze, Coord::default());

        assert!(found);
        assert_eq!(path.len(), 18);
        assert_eq!(path.pop_back(), Some(Coord { x: 0, y: 0, z: 0 }));
        assert_eq!(path.front(), Some(&Coord { x: 4, y: 4, z: 5 }));
    }
}
