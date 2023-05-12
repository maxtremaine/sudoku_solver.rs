#[test]
fn solves_puzzles() {
    let input_sudoku_file = String::from("  abc def ghi\n1 ___|___|___\n2 _3_|1_6|2_7\n3 6__|_3_|51_\n  -----------\n4 32_|__9|___\n5 __8|__5|7__\n6 ___|8__|_53\n  -----------\n7 _47|_9_|__8\n8 8_1|7_2|_9_\n9 ___|___|___");
    let output_sudoku_file = String::from("  abc def ghi\n1 712|954|836\n2 539|186|247\n3 684|237|519\n  -----------\n4 325|479|681\n5 198|365|724\n6 476|821|953\n  -----------\n7 247|593|168\n8 861|742|395\n9 953|618|472");
    assert_eq!(sudoku_solver_rs::solve_file(input_sudoku_file, false).unwrap(), output_sudoku_file);
}

#[test]
fn raises_errors() {
    let short_string = String::from("hello");
    let invalid_sudoku_file = String::from("  abc def ghi\n1 111|___|___\n2 _3_|1_6|2_7\n3 6__|_3_|51_\n  -----------\n4 32_|__9|___\n5 __8|__5|7__\n6 ___|8__|_53\n  -----------\n7 _47|_9_|__8\n8 8_1|7_2|_9_\n9 ___|___|___");
    assert_eq!(sudoku_solver_rs::solve_file(short_string, false).unwrap_err(),
        "A .sudoku file must be 167 characters long.");
    assert_eq!(sudoku_solver_rs::solve_file(invalid_sudoku_file, false).unwrap_err(),
        "The Sudoku file does not have a valid solution.");
}

#[test]
fn solves_arrays() {
    let start_values = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 1, 0, 6, 2, 0, 7, 6, 0, 0, 0, 3, 0, 5,
        1, 0, 3, 2, 0, 0, 0, 9, 0, 0, 0, 0, 0, 8, 0, 0, 5, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 5, 3, 0,
        4, 7, 0, 9, 0, 0, 0, 8, 8, 0, 1, 7, 0, 2, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let finish_values = [7, 1, 2, 9, 5, 4, 8, 3, 6, 5, 3, 9, 1, 8, 6, 2, 4, 7, 6, 8, 4, 2, 3, 7, 5,
        1, 9, 3, 2, 5, 4, 7, 9, 6, 8, 1, 1, 9, 8, 3, 6, 5, 7, 2, 4, 4, 7, 6, 8, 2, 1, 9, 5, 3, 2,
        4, 7, 5, 9, 3, 1, 6, 8, 8, 6, 1, 7, 4, 2, 3, 9, 5, 9, 5, 3, 6, 1, 8, 4, 7, 2];
    assert_eq!(sudoku_solver_rs::solve(start_values, false).unwrap(), finish_values);
}

