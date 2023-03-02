use crate::loopover::solver::movement;
use crate::loopover::structs::Board;
use crate::ErrorMessage;

pub fn solve_last_row(board: &mut Board, solved_board: &Board) {
    for x in 0..board.get_width() {
        let (mut pos_x, mut pos_y) = board
            .find_tile(solved_board.get_tile(x, board.get_height() - 1))
            .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardsMismatched.get_message()));

        if pos_x != x && pos_y == board.get_height() - 1 {
            (pos_x, pos_y) = movement::move_col(board, pos_x, pos_y, -1);
        }

        (pos_x, pos_y) = movement::move_row(board, pos_x, pos_y, calculate_offset(pos_x, x));

        movement::move_col(
            board,
            pos_x,
            pos_y,
            calculate_offset(pos_y, board.get_height() - 1),
        );
    }
}

pub fn solve_row(board: &mut Board, solved_board: &Board, y: usize) {
    for x in 0..board.get_width() {
        let (mut pos_x, mut pos_y) = board
            .find_tile(solved_board.get_tile(x, y))
            .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardsMismatched.get_message()));

        let mut helper_col_used = false;
        let first = x == 0;
        let last = x == board.get_width() - 1;
        let mut helper_col_x = 0;

        if !first && pos_y == y && pos_x != board.get_width() - 1 {
            // Move tile col down
            (pos_x, pos_y) = movement::move_col(board, pos_x, pos_y, 1);
            helper_col_used = true;
            helper_col_x = pos_x;
        }

        if last {
            if pos_x == x {
                (pos_x, pos_y) = movement::move_row(board, pos_x, pos_y, -1);
            }
            movement::move_col(board, board.get_width() - 1, y, calculate_offset(y, pos_y));
        }

        if !(first && pos_y == y) {
            // Move to elevator
            (pos_x, pos_y) = movement::move_row(
                board,
                pos_x,
                pos_y,
                calculate_offset(pos_x, board.get_width() - 1),
            );
        }

        // Move old tile col up IF HAD TO BEFORE
        if helper_col_used {
            movement::move_col(board, helper_col_x, 0, -1);
        }

        let mut elevator_offset = pos_y as isize;

        // Elevator up
        (pos_x, pos_y) = movement::move_col(board, pos_x, pos_y, calculate_offset(pos_y, y));

        if !last {
            elevator_offset -= pos_y as isize;
            // To Spot In Line
            movement::move_row(
                board,
                pos_x,
                pos_y,
                calculate_offset(pos_x, board.get_width() - 2),
            );
            // Elevator back down
            movement::move_col(board, board.get_width() - 1, 0, elevator_offset);
        }
    }
}

fn calculate_offset(current: usize, desired: usize) -> isize {
    desired as isize - current as isize
}
