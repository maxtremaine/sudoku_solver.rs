mod pure_functions;
mod puzzle_actions;

fn main() {
    let start_puzzle: [[u8; 9]; 9] = [
        [ 4, 1, 0, 0, 0, 0, 0, 0, 0 ],
        [ 0, 0, 3, 0, 0, 0, 0, 2, 9 ],
        [ 0, 0, 0, 0, 0, 4, 0, 6, 0 ],
        [ 0, 0, 0, 7, 0, 0, 0, 9, 0 ],
        [ 0, 0, 7, 4, 0, 0, 0, 0, 2 ],
        [ 0, 0, 0, 0, 0, 8, 0, 0, 5 ],
        [ 6, 7, 0, 0, 0, 1, 0, 0, 0 ],
        [ 0, 0, 9, 0, 2, 0, 0, 0, 3 ],
        [ 0, 3, 0, 0, 0, 9, 0, 5, 0 ]
    ];

    if !puzzle_actions::is_valid_puzzle(start_puzzle) {
        panic!("The start puzzle is not valid.")
    }
    
    let max_run_index = start_puzzle.iter()
        .fold(1, |mut acc, row| {
            for value in row {
                if value == &0 {
                    acc += 1
                }
            }
            acc
        });

    (1..max_run_index)
        .fold(start_puzzle, |working_branches, run_count| -> [[u8; 9]; 9] {
            
            println!("{}", run_count);
            [
                [ 4, 1, 0, 0, 0, 0, 0, 0, 0 ],
                [ 0, 0, 3, 0, 0, 0, 0, 2, 9 ],
                [ 0, 0, 0, 0, 0, 4, 0, 6, 0 ],
                [ 0, 0, 0, 7, 0, 0, 0, 9, 0 ],
                [ 0, 0, 7, 4, 0, 0, 0, 0, 2 ],
                [ 0, 0, 0, 0, 0, 8, 0, 0, 5 ],
                [ 6, 7, 0, 0, 0, 1, 0, 0, 0 ],
                [ 0, 0, 9, 0, 2, 0, 0, 0, 3 ],
                [ 0, 3, 0, 0, 0, 9, 0, 5, 0 ]
            ]
        });
}
