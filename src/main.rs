use std::env;

use input::read_from_args;

mod board;
mod input;

fn main() {
    let (input, solved) = read_from_args(env::args());
}
