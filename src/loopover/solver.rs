use crate::loopover::structs::Board;

mod instructions;
mod movement;

pub fn solve(board: &mut Board, solved_board: &Board) -> String {
    execute_instructions(board, solved_board);
    format!("{:?}", board.get_moves())
}

fn execute_instructions(board: &mut Board, solved_board: &Board) {
    instructions::solve_last_row(board, solved_board);
    for y in 0..board.get_height() - 2 {
        instructions::solve_row(board, solved_board, y);
    }
}
