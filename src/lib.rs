use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 256;
        let height = 128;

        let mut k = 1u128;
        let cells = (0..width * height)
            .map(|_| {
                k *= 1234567890;
                k %= 9999999999;
                if k % 5 == 0 {
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

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                match (cell, live_neighbors) {
                    (Cell::Alive, x) => {
                        next[idx] = if x < 2 || 3 < x {
                            Cell::Dead
                        } else {
                            Cell::Alive
                        }
                    }
                    (Cell::Dead, 3) => next[idx] = Cell::Alive,
                    _ => {}
                };
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl Universe {
    fn get_index(&self, row: usize, column: usize) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        for d_row in [-1, 0, 1] {
            for d_col in [-1, 0, 1] {
                if d_row == 0 && d_col == 0 {
                    continue;
                }

                let neighbor_row =
                    ((row as isize) + d_row + self.height as isize) as usize % self.height;
                let neighbor_col =
                    ((col as isize) + d_col + self.width as isize) as usize % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += if self.cells[idx] == Cell::Dead { 0 } else { 1 };
            }
        }

        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { ' ' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
