# sudoku_solver.rs

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
6.      // Put your start puzzle here.
7.      let start_puzzle: [[u8; 9]; 9] = [
8.  //col:    a  b  c  d  e  f  g  h  i      row:
9.          [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ], // 1
10.         [ 0, 3, 0, 1, 0, 6, 2, 0, 7 ], // 2
11.         [ 6, 0, 0, 0, 3, 0, 5, 1, 0 ], // 3
12.         [ 3, 2, 0, 0, 0, 9, 0, 0, 0 ], // 4
13.         [ 0, 0, 8, 0, 0, 5, 7, 0, 0 ], // 5
14.         [ 0, 0, 0, 8, 0, 0, 0, 5, 3 ], // 6
15.         [ 0, 4, 7, 0, 9, 0, 0, 0, 8 ], // 7
16.         [ 8, 0, 1, 7, 0, 2, 0, 9, 0 ], // 8
17.         [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ]  // 9
18.     ];
```

Alright, now you're ready to run.

```sh
cargo run
```

The completed puzzle will be printed to the terminal after all of the cells are filled.