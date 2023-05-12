# sudoku_solver_rs

Solves sudoku so I don't have to, in Rust!

## Reason and Philosophy

I haven't been able to keep up with coding as much as I would have, but I do spend an unnecessary amount of time doing sudoku puzzles, so I thought, "Hey, I should combine the two." I have tried to make the program easy to interact with, and be highly efficient without becoming bloated with different systems/tactics for solving puzzles. I hope that at least the .sudoku file type and interpreter functions are useful to other people who want to play with sudoku as a platform.

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

The completed puzzle will be printed to the terminal and added to io/finish.sudoku after all of the cells are filled.

You can also use the _solve_ function from the sudoku_solver_rs library if you want to solve puzzles from .sudoku files in your own program.

## To Do

- [ ] Use question mark operator for error propagation in lib.rs
