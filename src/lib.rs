mod sudoku;
mod blank_cell;
mod pure_functions;

use sudoku::Sudoku;

// TODO: Add solve for an array of u8s.
pub fn solve_file(input_sudoku_file: String, verbose: bool) -> Result<String, &'static str> {
    if verbose {
        println!("\n");
    }
    let start_puzzle = Sudoku::from_file(input_sudoku_file)?;
    let output = start_puzzle.get_valid_solutions(true);
    let output_sudoku_file = output[0].to_string();
    if verbose {
        println!("\nFinished Puzzle:\n\n{}\n", output_sudoku_file);
    }
    Ok(output_sudoku_file)
}