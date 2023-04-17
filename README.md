# sudoku_solver_rs

Solves sudoku so I don't have to, in Rust!

## How to solve puzzles.

First, you need to have [rust](https://www.rust-lang.org/).

Now, you can clone this repository in a convenient directory. I will use Desktop as an example.

```sh
cd Desktop
git clone https://github.com/maxtremaine/sudoku_solver.git
```

Then, you can add your starting puzzle in src/main.rs.

```rs
12.     // Put your start puzzle here.
13.     let start_puzzle: [u8; 81] = [
14.     //col: a  b  c  d  e  f  g  h  i    row:
15.            0, 0, 0, 0, 0, 0, 0, 0, 0, // 1
16.            0, 3, 0, 1, 0, 6, 2, 0, 7, // 2
17.            6, 0, 0, 0, 3, 0, 5, 1, 0, // 3
18.            3, 2, 0, 0, 0, 9, 0, 0, 0, // 4
19.            0, 0, 8, 0, 0, 5, 7, 0, 0, // 5
20.            0, 0, 0, 8, 0, 0, 0, 5, 3, // 6
21.            0, 4, 7, 0, 9, 0, 0, 0, 8, // 7
22.            8, 0, 1, 7, 0, 2, 0, 9, 0, // 8
23.            0, 0, 0, 0, 0, 0, 0, 0, 0  // 9
24.     ];
```

Alright, now you're ready to run.

```sh
cargo run
```

The completed puzzle will be printed to the terminal after all of the cells are filled.

## To Do

- [ ] Update README to include reasoning and philosophy
