use std::time::Instant;
use std::fs::{read_to_string, write};

fn main() {

    println!("Starting sudoku_solver_rs");

    let t0 = Instant::now();

    let input_sudoku_file = read_to_string("./io/start.sudoku")
        .expect("Unable to read file.");

    let output_sudoku_file = sudoku_solver_rs::solve(input_sudoku_file, true);

    write("./io/finish.sudoku", output_sudoku_file.unwrap()).expect("Unable to write file.");

    let elapsed = t0.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
