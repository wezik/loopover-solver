use crate::loopover::structs::Board;

mod instructions;
mod movement;

pub fn solve(unsolved_board: &mut Board, solved_board: &Board) -> String {
    execute_instructions(unsolved_board, solved_board);
    format!("{:?}", unsolved_board.get_moves())
}

fn execute_instructions(unsolved_board: &mut Board, solved_board: &Board) {
    instructions::solve_bottom_row(unsolved_board, solved_board);
    for y in 0..unsolved_board.get_height() - 2 {
        instructions::solve_board_row(unsolved_board, solved_board, y);
    }
}
