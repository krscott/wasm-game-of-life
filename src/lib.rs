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

impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Dead => '◻',
            Cell::Alive => '◼',
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl Cell {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '◻' => Some(Cell::Dead),
            '◼' => Some(Cell::Alive),
            _ => None,
        }
    }

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                write!(f, "{}", self.get_cell(r as i32, c as i32))?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

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

    pub fn random(width: u32, height: u32) -> Self {
        let mut univ = Self::empty(width, height);

        for i in 0..univ.cells.len() {
            univ.cells.set(i, Cell::random().into())
        }

        univ
    }

    pub fn from_str(s: &str) -> Self {
        let s = s.trim();

        let grid: Vec<Vec<Cell>> = s
            .split('\n')
            .map(|line| line.chars().filter_map(|ch| Cell::from_char(ch)).collect())
            .collect();

        let height = grid.len() as u32;
        let width = grid.iter().map(|row| row.len()).max().unwrap_or(1) as u32;

        let mut univ = Self::empty(width, height);

        for (r, row) in grid.into_iter().enumerate() {
            for (c, cell) in row.into_iter().enumerate() {
                univ.set_cell(r as i32, c as i32, cell);
            }
        }

        univ
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
