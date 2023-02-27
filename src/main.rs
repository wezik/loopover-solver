use std::env;

use crate::modules::loopover_error::ErrorMessage;
use crate::modules::{loopover_mapper, solver_service};

mod modules;
#[cfg(test)]
mod modules_test;

fn main() {
    let (starting, solved) = read_args();
    let (mut unsolved_board, solved_board) = loopover_mapper::map_args_to_board(starting, solved);
    solver_service::solve(&mut unsolved_board, &solved_board);
}

fn read_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    let starting = args
        .get(1)
        .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardsMissing.get_message()))
        .to_string();
    let solved = args
        .get(2)
        .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardsMissing.get_message()))
        .to_string();

    (starting, solved)
}
