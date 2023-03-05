mod pure_functions;
mod puzzle_actions;

use std::time::Instant;

fn main() {
    // Start the clock.
    let t0 = Instant::now();
    
    // Put your start puzzle here.
    let start_puzzle: [[u8; 9]; 9] = [
    //col:a  b  c  d  e  f  g  h  i      row:
        [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ], // 1
        [ 0, 3, 0, 1, 0, 6, 2, 0, 7 ], // 2
        [ 6, 0, 0, 0, 3, 0, 5, 1, 0 ], // 3
        [ 3, 2, 0, 0, 0, 9, 0, 0, 0 ], // 4
        [ 0, 0, 8, 0, 0, 5, 7, 0, 0 ], // 5
        [ 0, 0, 0, 8, 0, 0, 0, 5, 3 ], // 6
        [ 0, 4, 7, 0, 9, 0, 0, 0, 8 ], // 7
        [ 8, 0, 1, 7, 0, 2, 0, 9, 0 ], // 8
        [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ]  // 9
    ];

    // Stop if the input is not valid.
    if !puzzle_actions::is_valid_puzzle(start_puzzle) {
        panic!("The start puzzle is not valid.")
    }
    
    // Count the number of blank cells.
    let max_run_index: u8 = start_puzzle.iter()
        .fold(0, |mut acc, row| {
            for value in row {
                if value == &0 {
                    acc += 1
                }
            }
            acc
        });

    // Iterate over the blank cell count.
    let output = (1..=max_run_index)
        .fold(Vec::from([start_puzzle]), |working_branches, run_count| -> Vec<[[u8; 9]; 9]> {
            let new_working_branches: Vec<[[u8; 9]; 9]> = working_branches.iter()

                // Add new branches based on degrees of freedom, narrowing as branches collapse.
                .fold(Vec::new(), |mut valid_new_branches, old_branch| -> Vec<[[u8; 9]; 9]> {

                    // Create Cell objects for each blank cell (0 value).
                    let mut blank_cells: Vec<puzzle_actions::Cell> = old_branch.iter().enumerate()
                        .fold(Vec::new(), |mut new_cells, (y, row)| -> Vec<puzzle_actions::Cell> {
                            row.iter().enumerate().for_each(|(x, cell_value)| {
                                if cell_value == &0 {
                                    new_cells.push(puzzle_actions::Cell::new((y.try_into().unwrap(), x.try_into().unwrap()), *old_branch))
                                }
                            });
                            new_cells
                        });
                    
                    // Sort by degrees of freedom, ascending.
                    blank_cells.sort_by(|a, b| a.possible_values.len().cmp(&b.possible_values.len()));
                    
                    let new_branches: Vec<[[u8; 9];9]> = blank_cells[0].possible_values.iter()
                        .map(|value| {
                            puzzle_actions::change_cell(blank_cells[0].coords, *value, *old_branch)
                        }).collect();

                    new_branches.iter().for_each(|branch| valid_new_branches.push(*branch));

                    valid_new_branches
                });

            println!("Completed run {} of {} with {} branches.", run_count, max_run_index, new_working_branches.len());
            new_working_branches
        });
    
    println!("\nFinished Puzzle\n===============");
    output[0].iter().for_each(|row| println!("{:?}", row));

    let elapsed = t0.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
