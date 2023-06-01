//! # Sudoku Solver (Rust)
//!
//! Solves sudoku puzzles so I don't have to, in Rust. This crate accepts input as a string or
//! array and returns a completed sudoku puzzle. This crate can also be run with input from the
//! /io folder.

mod sudoku;
mod blank_cell;
mod pure_functions;

use sudoku::Sudoku;

/// Solves sudoku puzzles with a .sudoku file formatted string as input.
// TODO: Add input enum and generic solve function.
pub fn solve_file(input_sudoku_file: String, verbose: bool) -> Result<String, &'static str> {
    if verbose {
        println!("\n");
    }
    let output_file = Sudoku::from_file(input_sudoku_file)?
        .get_valid_solutions(verbose)[0]
        .to_string();
    if verbose {
        println!("\nFinished Puzzle:\n\n{}\n", output_file);
    }
    Ok(output_file)
}

/// Solves sudoku puzzles with an array of 81 numbers as input.
///
/// # Input
///
/// The input array starts at the top left of the sudoku puzzle progressing across and down, where
/// the last number in the array is the bottom right. Blank cells are "0".
pub fn solve(input_values: [u8; 81], verbose: bool) -> Result<[u8; 81], &'static str> {
    if verbose {
        println!("\n");
    }
    let output_values = Sudoku::from(input_values)?
        .get_valid_solutions(verbose)[0]
        .numbers;
    if verbose {
        println!("\nFinished Puzzle:\n\n{:?}\n", output_values);
    }
    Ok(output_values)
}