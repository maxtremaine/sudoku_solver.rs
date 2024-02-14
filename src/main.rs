use std::time::Instant;
use std::fs::{read_to_string, write};

fn main() {

    println!("Starting sudoku_solver_rs");

    let t0 = Instant::now();

    let input_sudoku_file = read_to_string("./io/start.sudoku")
        .expect("Unable to read file.");

    let output_sudoku_file = sudoku_solver_rs::solve_file(input_sudoku_file, true)
        .unwrap();

    write("./io/finish.sudoku", output_sudoku_file)
        .expect("Unable to write file.");

    let elapsed_string = format!("{:.2?}", t0.elapsed());
    println!("Elapsed: {elapsed_string}");
}
