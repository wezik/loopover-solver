use std::env;

use crate::input::read_from_args;
use crate::solver::generate_solve;

mod board;
mod input;
mod solver;

fn main() {
    let board = read_from_args(env::args());
    let moves = generate_solve(board);
    println!("{:?}", moves);
}
