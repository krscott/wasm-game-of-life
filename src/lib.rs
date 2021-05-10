mod utils;

use std::fmt;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Cell::Dead => '◻',
            Cell::Alive => '◼',
        };
        write!(f, "{}", symbol)
    }
}

impl From<Cell> for bool {
    fn from(cell: Cell) -> Self {
        cell == Cell::Alive
    }
}

impl From<bool> for Cell {
    fn from(x: bool) -> Self {
        match x {
            true => Cell::Alive,
            false => Cell::Dead,
        }
    }
}

impl Cell {
    pub fn random() -> Self {
        #[allow(unused_unsafe)]
        let f = unsafe { js_sys::Math::random() };
        if f < 0.5 {
            Cell::Dead
        } else {
            Cell::Alive
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: i32, column: i32) -> usize {
        let row = row.rem_euclid(self.height as i32);
        let column = column.rem_euclid(self.width as i32);

        (row * (self.width as i32) + column) as usize
    }

    fn get_coord(&self, index: usize) -> (i32, i32) {
        (
            (index / self.width as usize) as i32,
            (index % self.width as usize) as i32,
        )
    }

    fn get_cell(&self, row: i32, column: i32) -> Cell {
        self.cells[self.get_index(row, column)].into()
    }

    fn set_cell(&mut self, row: i32, column: i32, cell: Cell) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, cell.into());
    }

    fn live_neighbor_count(&self, row: i32, column: i32) -> u8 {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter(|(dr, dc)| self.get_cell(row + *dr, column + *dc) == Cell::Alive)
        .count() as u8

        // (-1..=1)
        //     .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
        //     .filter(|(dr, dc)| (*dr != 0 || *dc != 0))
        //     .filter(|(dr, dc)| self.get_cell(row + dr, column + dc) == Cell::Alive)
        //     .count() as u8
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn empty(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, Cell::Dead.into());
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn with_single_spaceship(width: u32, height: u32) -> Self {
        let mut univ = Self::empty(width, height);

        let mid_row = height as i32 / 2 - 2;
        let mid_col = width as i32 / 2 - 2;
        let glider = "\
            1__1_\n\
            ____1\n\
            1___1\n\
            _1111\n\
        ";

        univ.insert_from_str(mid_row, mid_col, glider);

        univ
    }

    pub fn random(width: u32, height: u32) -> Self {
        let mut univ = Self::empty(width, height);

        for i in 0..univ.cells.len() {
            univ.cells.set(i, Cell::random().into())
        }

        univ
    }

    pub fn insert_from_str(&mut self, row: i32, col: i32, cells_str: &str) {
        for (dr, line) in cells_str.split('\n').enumerate() {
            let line = line.trim_matches('\r');

            for (dc, ch) in line.chars().enumerate() {
                let cell = if ch == ' ' || ch == '0' || ch == '_' {
                    Cell::Dead
                } else {
                    Cell::Alive
                };

                self.set_cell(row + dr as i32, col + dc as i32, cell);
            }
        }
    }

    pub fn tick(&mut self) {
        let mut new_cells = FixedBitSet::with_capacity((self.width * self.height) as usize);

        for i in 0..self.cells.len() {
            let (r, c) = self.get_coord(i);
            let cell = self.get_cell(r, c);
            let live_neighbors = self.live_neighbor_count(r, c);

            let new_cell = match (cell, live_neighbors) {
                (Cell::Alive, 2) | (_, 3) => Cell::Alive,
                _ => Cell::Dead,
            };

            new_cells.set(i, new_cell.into());
        }

        self.cells = new_cells;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}
