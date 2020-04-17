//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use conway::{Board, CellState};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}


#[wasm_bindgen_test]
fn create_board() {
    let board = Board::new(1, 1);
}

#[wasm_bindgen_test]
fn set_a_cell() {
    let mut board = Board::new(2, 2);
    board.toggle_cell(0, 0);
    let cells = board.cells();
    assert_eq!(cells[0], 1);
}