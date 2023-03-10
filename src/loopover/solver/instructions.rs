use crate::loopover::structs::Board;

mod rows;

pub fn solve_row(unsolved_board: &mut Board, solved_board: &Board, row_index: usize) {
    let max_y = unsolved_board.get_height() - 1;
    match row_index {
        i if i == max_y => {
            rows::solve_bottom_row(unsolved_board, solved_board);
        }
        i if i == max_y - 1 => {
            rows::solve_second_from_last_row(unsolved_board, solved_board);
        }
        i => {
            rows::solve_normal_row(unsolved_board, solved_board, i);
        }
    }
}
