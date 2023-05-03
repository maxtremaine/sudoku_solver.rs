# sudoku_solver_rs

Solves sudoku so I don't have to, in Rust!

## How to solve puzzles.

First, you need to have [rust](https://www.rust-lang.org/).

Now, you can clone this repository in a convenient directory. I will use Desktop as an example.

```sh
cd Desktop
git clone https://github.com/maxtremaine/sudoku_solver.git
```

Next, update the file in io/start.sudoku to reflect your puzzle to solve.

```
  abc def ghi
1 ___|___|___
2 _3_|1_6|2_7
3 6__|_3_|51_
  -----------
4 32_|__9|___
5 __8|__5|7__
6 ___|8__|_53
  -----------
7 _47|_9_|__8
8 8_1|7_2|_9_
9 ___|___|___
```

Alright, now you're ready to run.

```sh
cargo run
```

The completed puzzle will be printed to the terminal after all of the cells are filled.

## To Do

- [ ] Update README to include reasoning and philosophy
