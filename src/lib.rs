mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::{fmt, error};
use std::cmp::{max, min};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Board {
    rows: u16,
    cols: u16,
    cells: Vec<AutomataCell>
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(rows: u16, cols: u16) -> Board {
        Board{
            rows: rows,
            cols: cols,
            cells: vec![AutomataCell{ state: CellState::Dead }; (rows * cols) as usize]
        }
    }

    pub fn set_cell(&mut self, x: u16, y: u16, other: CellState) -> Option<CellState> {
        let index = self.index(x, y);
        self.cells.get_mut(index).map(|cell| {
            cell.state = other;
            return cell.state;
        })
    }

    pub fn cells(&self) -> Vec<u8> {
        self.cells.iter().map(|cell| (*cell).into()).collect()
    }

    fn index(&self, x: u16, y: u16) -> usize {
        (x * self.rows + y) as usize
    }

    pub fn update(&self) -> Board {
        let mut new_cells: Vec<AutomataCell> = vec!();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut neighbors: [AutomataCell; 8] = [AutomataCell{ state: CellState::Dead }; 8];
                for nj in max(0, j - 1)..min(j + i, self.cols) {
                    for ni in max(0, i - 1)..min(self.rows, i + 1) {
                        if ni != i && nj != j {
                            neighbors[i as usize] = self.cells[self.index(ni, nj)]
                        }
                    }
                }

                let index = self.index(i, j);
                new_cells.push(self.cells[index].update_cell(&neighbors[..]));
            }
        }

        Board {
            rows: self.rows,
            cols: self.cols,
            cells: new_cells
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum CellState {
    Dead,
    Alive
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct AutomataCell {
    pub state: CellState
}


impl From<AutomataCell> for u8 {
    fn from(cell: AutomataCell) -> u8 {
        match cell.state {
            CellState::Alive => 1,
            CellState::Dead => 0
        }
    }
}

impl From<AutomataCell> for bool {
    fn from(cell: AutomataCell) -> bool {
        match cell.state {
            CellState::Alive => true,
            CellState::Dead => false
        }
    }
}

#[wasm_bindgen]
impl AutomataCell {
    fn update_cell(&self, neighbors: &[AutomataCell]) -> AutomataCell {
        let mut count = 0;
        for neighbor in neighbors {
            count += match neighbor.state {
                CellState::Alive => 1,
                CellState::Dead => 0
            }
        }
    
        AutomataCell{
            state: match (self.state, count) {
                (CellState::Alive, 2..=3) => CellState::Alive,
                (CellState::Dead, 3) => CellState::Alive,
                _ => CellState::Dead
            }
        }
    }
}