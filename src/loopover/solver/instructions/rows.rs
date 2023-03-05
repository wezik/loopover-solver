use crate::loopover::structs::Board;
use crate::ErrorMessage;

mod tiles;

pub fn solve_bottom_row(
    unsolved_board: &mut Board,
    solved_board: &Board,
    max_x: usize,
    max_y: usize,
) {
    for solved_x in 0..=max_x {
        let solved_tile = solved_board.get_tile(solved_x, max_y);
        let (x, y) = handle_mismatch(unsolved_board.find_tile(solved_tile));
        tiles::solve_bottom_tile(unsolved_board, x, y, solved_x, max_y);
    }
}

pub fn solve_normal_row(
    unsolved_board: &mut Board,
    solved_board: &Board,
    max_x: usize,
    y_index: usize,
) {
    for solved_x in 0..=max_x {
        let solved_tile = solved_board.get_tile(solved_x, y_index);
        let (pos_x, pos_y) = handle_mismatch(unsolved_board.find_tile(solved_tile));
        match solved_x {
            0 => {
                tiles::solve_normal_tile_first(unsolved_board, pos_x, pos_y, y_index, max_x);
            }
            i if i == max_x => {
                tiles::solve_normal_tile_last(unsolved_board, pos_x, pos_y, y_index, max_x);
            }
            _ => {
                tiles::solve_normal_tile(unsolved_board, pos_x, pos_y, y_index, max_x);
            }
        }
    }
}

pub fn solve_second_from_last_row(
    unsolved_board: &mut Board,
    solved_board: &Board,
    max_x: usize,
    max_y: usize,
) {
    let solved_tile = solved_board.get_tile(0, max_y - 1);
    let (pos_x, pos_y) = handle_mismatch(unsolved_board.find_tile(solved_tile));

    tiles::solve_second_from_last_tile_first(unsolved_board, pos_x, pos_y, max_x, max_y);

    for solved_x in 1..max_x {
        let solved_tile = solved_board.get_tile(solved_x, max_y - 1);
        let (pos_x, pos_y) = handle_mismatch(unsolved_board.find_tile(solved_tile));
        tiles::solve_second_from_last_tile(unsolved_board, pos_x, pos_y, max_x, max_y);
    }
}

fn handle_mismatch(option: Option<(usize, usize)>) -> (usize, usize) {
    option.unwrap_or_else(|| panic!("{}", ErrorMessage::BoardsMismatched.get_message()))
}
