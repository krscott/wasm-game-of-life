mod utils;

use std::fmt;

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
    cells: Vec<Cell>,
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
        self.cells[self.get_index(row, column)]
    }

    fn set_cell(&mut self, row: i32, column: i32, cell: Cell) {
        let idx = self.get_index(row, column);
        self.cells[idx] = cell;
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

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.chunks(self.width as usize) {
            for &cell in line {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn with_test_pattern(width: u32, height: u32) -> Self {
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn empty(width: u32, height: u32) -> Self {
        let cells = (0..width * height).map(|_| Cell::Dead).collect();

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
        let cells = (0..width * height).map(|_| Cell::random()).collect();

        Universe {
            width,
            height,
            cells,
        }
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
        self.cells = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                let (r, c) = self.get_coord(i);
                let live_neighbors = self.live_neighbor_count(r, c);

                match (cell, live_neighbors) {
                    (Cell::Alive, 2) | (_, 3) => Cell::Alive,
                    _ => Cell::Dead,
                }
            })
            .collect();
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}
