use crate::loopover::structs::Board;

mod instructions;
mod movement;

pub fn solve(board: &mut Board, solved_board: &Board) -> Vec<String> {
    instructions::solve_last_row(board, solved_board);
    Vec::new()
}
