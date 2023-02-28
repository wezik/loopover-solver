use crate::ErrorMessage;
use crate::loopover::solver::movement;
use crate::loopover::structs::Board;

pub fn solve_last_row(board: &mut Board, solved_board: &Board) {
    for x in 0..board.get_width() {
        let (mut pos_x, mut pos_y) = board
            .find_tile(solved_board.get_tile(x, board.get_height() - 1))
            .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardsMismatched.get_message()));

        if pos_x != x && pos_y == board.get_height() - 1 {
            (pos_x, pos_y) = movement::move_col(board, pos_x, pos_y, -1);
        }

        let x_offset: isize = x as isize - pos_x as isize;
        (pos_x, pos_y) = movement::move_row(board, pos_x, pos_y, x_offset);

        let y_offset: isize = board.get_height() as isize - 1 - pos_y as isize;
        movement::move_col(board, pos_x, pos_y, y_offset);
    }
}
