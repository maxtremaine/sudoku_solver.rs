mod sudoku;
mod blank_cell;
mod pure_functions;

use std::time::Instant;
use sudoku::Sudoku;

fn main() {
    // Start the clock.
    let t0 = Instant::now();
    
    // Put your start puzzle here.
    let starting_grid: [u8; 81] = [
    //col: a  b  c  d  e  f  g  h  i    row:
           0, 0, 0, 0, 0, 0, 0, 0, 0, // 1
           0, 3, 0, 1, 0, 6, 2, 0, 7, // 2
           6, 0, 0, 0, 3, 0, 5, 1, 0, // 3
           3, 2, 0, 0, 0, 9, 0, 0, 0, // 4
           0, 0, 8, 0, 0, 5, 7, 0, 0, // 5
           0, 0, 0, 8, 0, 0, 0, 5, 3, // 6
           0, 4, 7, 0, 9, 0, 0, 0, 8, // 7
           8, 0, 1, 7, 0, 2, 0, 9, 0, // 8
           0, 0, 0, 0, 0, 0, 0, 0, 0  // 9
    ];

    let start_puzzle = Sudoku{ numbers: starting_grid };

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
