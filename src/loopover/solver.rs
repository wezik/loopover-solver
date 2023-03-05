use crate::loopover::structs::Board;

mod instructions;
mod movement;

pub fn solve(unsolved_board: &mut Board, solved_board: &Board) -> String {
    instructions::solve_row(
        unsolved_board,
        solved_board,
        unsolved_board.get_height() - 1,
    );

    for y in 0..unsolved_board.get_height() - 1 {
        instructions::solve_row(unsolved_board, solved_board, y);
    }

    format!("{:?}", unsolved_board.get_moves())
}
