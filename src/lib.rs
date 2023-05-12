mod sudoku;
mod blank_cell;
mod pure_functions;

use sudoku::Sudoku;

// TODO: Add solve for an array of u8s.
pub fn solve(input_sudoku_file: String, verbose: bool) -> Result<String, &'static str> {
    if verbose {
        println!("\n");
    }
    let start_puzzle = Sudoku::from(input_sudoku_file)?;
    let max_run_index = start_puzzle.get_blank_cells().len();
    
    // Add to working puzzles and collapse when no options are available.
    let output: Vec<Sudoku> = (1..=max_run_index)
        .fold(Vec::from([start_puzzle]), |working_branches, run_count| {
            let new_working_branches = working_branches.iter()
                .fold(Vec::new(), |mut new_branches, current_branch| -> Vec<Sudoku> {
                    let blank_cells = current_branch.get_blank_cells();
                    let lowest_cell = &blank_cells[0];
                    lowest_cell.possible_values.iter().for_each(|possible_value| {
                        let new_branch = current_branch.change_cell(lowest_cell.index, *possible_value);
                        new_branches.push(new_branch);
                    });
                    new_branches
                });
            if verbose {
                println!("Completed run {} with {} branches.", run_count, new_working_branches.len());
            }
            new_working_branches
        });

    let output_sudoku_file = output[0].to_string();
    if verbose {
        println!("\nFinished Puzzle:\n\n{}\n", output_sudoku_file);
    }
    Ok(output_sudoku_file)
}