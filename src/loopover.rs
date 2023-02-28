use crate::ErrorMessage;

pub mod error;
#[cfg(test)]
mod loopover_tests;
mod mapper;
mod solver;
mod structs;

pub fn solve(args: std::env::Args) -> Vec<String> {
    let (mut current_state, solved_state) = mapper::map_args_to_boards(args)
        .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardsMismatched.get_message()));
    solver::solve(&mut current_state, &solved_state)
}