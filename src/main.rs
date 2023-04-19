mod sudoku;
mod blank_cell;
mod pure_functions;

use std::time::Instant;
use std::fs::read_to_string;
use sudoku::Sudoku;

fn main() {
    // Start the clock.
    let t0 = Instant::now();

    let start_file: String = read_to_string("./io/start.sudoku")
        .expect("Unable to read file.");

    let start_puzzle = Sudoku::from(start_file);

    // Stop if the input is not valid.
    if !start_puzzle.is_valid() {
        panic!("The start puzzle is not valid.")
    }
    
    // Count the number of blank cells.
    let max_run_index: u8 = start_puzzle.get_blank_cells().len().try_into().unwrap();

    let output: Vec<Sudoku> = (1..=max_run_index)
        .fold(Vec::from([start_puzzle]), |working_branches, run_count| -> Vec<Sudoku> {
            let new_working_branches = working_branches.iter().fold(Vec::new(), |mut new_branches, current_branch| -> Vec<Sudoku> {
                let blank_cells = current_branch.get_blank_cells();
                let lowest_cell = &blank_cells[0];
                lowest_cell.possible_values.iter().for_each(|possible_value| {
                    new_branches.push(current_branch.change_cell(lowest_cell.index, *possible_value));
                });
                new_branches
            });
            println!("Completed run {} with {} branches.", run_count, new_working_branches.len());
            new_working_branches
        });

    println!("Finished Puzzle:\n{:?}", output[0].numbers);

    let elapsed = t0.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
