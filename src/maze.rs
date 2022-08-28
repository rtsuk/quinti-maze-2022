use rand_chacha::rand_core::SeedableRng;
use rand_core::RngCore;

#[derive(Debug, Default, Clone, Copy)]
pub struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    pub east: bool,
    pub west: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub down: bool,
}

impl Cell {
    #[allow(unused)]
    pub fn full() -> Self {
        Self {
            east: true,
            west: true,
            north: true,
            south: true,
            up: true,
            down: true,
        }
    }

    #[allow(unused)]
    pub fn random() -> Self {
        let mut gen = rand_chacha::ChaCha8Rng::seed_from_u64(11);

        let v = gen.next_u64();
        Self {
            east: v & 0b00_0001 != 0,
            west: v & 0b00_0010 != 0,
            north: v & 0b00_0100 != 0,
            south: v & 0b00_1000 != 0,
            up: v & 0b01_0000 != 0,
            down: v & 0b10_0000 != 0,
        }
    }
}

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
    pub fn get_cell(&self, coord: &Coord) -> Cell {
        self.cells[coord.z][coord.y][coord.x]
    }
}
