use crate::modules::loopover::Board;
use crate::ErrorMessage;

pub fn solve(board: &mut Board, solved_board: &Board) -> Vec<String> {
    solve_last_row(board, solved_board);
    Vec::new()
}

fn solve_last_row(board: &mut Board, solved_board: &Board) {
    for x in 0..board.get_width() {
        let (mut pos_x, mut pos_y) = board
            .find_tile(solved_board.get_tile(x, board.get_height() - 1))
            .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardsMismatched.get_message()));

        if pos_x != x && pos_y == board.get_height() - 1 {
            (pos_x, pos_y) = move_col(board, pos_x, pos_y, -1);
        }

        let x_offset: isize = x as isize - pos_x as isize;
        (pos_x, pos_y) = move_row(board, pos_x, pos_y, x_offset);

        let y_offset: isize = board.get_height() as isize - 1 - pos_y as isize;
        move_col(board, pos_x, pos_y, y_offset);
    }
}

fn move_col(board: &mut Board, x: usize, y: usize, distance: isize) -> (usize, usize) {
    if distance > 0 {
        board.move_col_down(x, distance as usize);
    } else {
        board.move_col_up(x, distance.unsigned_abs());
    }
    (x, (y as isize + distance) as usize)
}

fn move_row(board: &mut Board, x: usize, y: usize, distance: isize) -> (usize, usize) {
    if distance > 0 {
        board.move_row_right(y, distance as usize);
    } else {
        board.move_row_left(y, distance.unsigned_abs());
    }
    ((x as isize + distance) as usize, y)
}
