#[test]
fn solves_puzzles() {
    let input_sudoku_file = String::from("  abc def ghi\n1 ___|___|___\n2 _3_|1_6|2_7\n3 6__|_3_|51_\n  -----------\n4 32_|__9|___\n5 __8|__5|7__\n6 ___|8__|_53\n  -----------\n7 _47|_9_|__8\n8 8_1|7_2|_9_\n9 ___|___|___");
    let output_sudoku_file = String::from("  abc def ghi\n1 712|954|836\n2 539|186|247\n3 684|237|519\n  -----------\n4 325|479|681\n5 198|365|724\n6 476|821|953\n  -----------\n7 247|593|168\n8 861|742|395\n9 953|618|472");
    assert_eq!(sudoku_solver_rs::solve(input_sudoku_file, false).unwrap(), output_sudoku_file);
}

#[test]
fn raises_errors() {
    let short_string = String::from("hello");
    let invalid_sudoku_file = String::from("  abc def ghi\n1 111|___|___\n2 _3_|1_6|2_7\n3 6__|_3_|51_\n  -----------\n4 32_|__9|___\n5 __8|__5|7__\n6 ___|8__|_53\n  -----------\n7 _47|_9_|__8\n8 8_1|7_2|_9_\n9 ___|___|___");
    assert_eq!(sudoku_solver_rs::solve(short_string, false).unwrap_err(),
        "A .sudoku file must be 167 characters long.");
    assert_eq!(sudoku_solver_rs::solve(invalid_sudoku_file, false).unwrap_err(),
    "The Sudoku file does not have a valid solution.");
}