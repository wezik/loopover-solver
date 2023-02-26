use crate::board::Board;

pub fn generate_solve(board: &Board) -> Vec<String> {
    let mut board = board.clone();
    println!("{:?}", board);
    solve_last_row(&mut board);
    println!("{:?}", board);
    Vec::new()
}

fn solve_last_row(board: &mut Board) {
    for x in 0..board.get_width() {
        let (mut pos_x, mut pos_y) = board
            .find_tile(board.get_solved_tile(x, board.get_height() - 1))
            .expect("boards don't match");

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
