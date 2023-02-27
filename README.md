# loopover-solver
This is a Rust application that solves loopover puzzle.
It takes in unsolved and solved state as an input, and returns a list of instruction that will lead from one state to the other

## Usage
Application can be run from the command line, and takes in two arguments:

- Unsolved, a string represting the current state of the puzzle.
- Solved, a string represting the desired state of the puzzle

For example:

`"I O J P-C E H D-M N K L-B F G A" "A B C D-E F G H-I J K L-M N O P"`

The two strings should be space-seperated and use '-' character to seperate rows.

The result output is an array of strings representing the instructions needed to solve the puzzle. Each instruction is represented by a letter inidcating the direction of the slide, followed by a number indicating the row or column index. Directions are:
- `R`: slide to the right
- `L`: slide to the left
- `U`: slide up
- `D`: slide down

For example:

```["R1", "R3", "L1", "U3", "D0"]```

This means: 
- 1st instruction is to slide the second (since we index from 0) row to the right. 
- 2nd instruction is to slide the index 3 row to the right.
- 3rd instruction is to slide the index 1 row to the left.
- 4th instruction is to slide the index 2 column up
- 5th instruction is to slide the index 0 column down.

## Building and running
#### Requirements
- Rust installed
#### Build
Once you have rust installed on your machine you can run
```cargo build --release```
This will build the application in release mode, which will optimize the performance and resulting executable.
#### Run
After all of that you can run the application with `cargo run` command or run the final executable that will be located in `/target/release`

```cargo run --release "I O J P-C E H D-M N K L-B F G A" "A B C D-E F G H-I J K L-M N O P"```
