use crate::loopover::structs::Board;

pub fn move_col(board: &mut Board, x: usize, y: usize, distance: isize) -> (usize, usize) {
    if distance > 0 {
        board.move_col_down(x, distance as usize);
    } else {
        board.move_col_up(x, distance.unsigned_abs());
    }
    (x, (y as isize + distance) as usize)
}

pub fn move_row(board: &mut Board, x: usize, y: usize, distance: isize) -> (usize, usize) {
    if distance > 0 {
        board.move_row_right(y, distance as usize);
    } else {
        board.move_row_left(y, distance.unsigned_abs());
    }
    ((x as isize + distance) as usize, y)
}
