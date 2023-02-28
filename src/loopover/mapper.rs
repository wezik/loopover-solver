use crate::loopover::structs::{Board, Tile};
use crate::ErrorMessage;

pub fn map_args_to_boards(args: std::env::Args) -> Option<(Board, Board)> {
    let input = map_args_to_strings(args.collect());
    let current_board = map_tiles_to_board(map_string_to_tiles(input.0));
    let solved_board = map_tiles_to_board(map_string_to_tiles(input.1));
    if !valid_boards(&current_board, &solved_board) {
        return None;
    }
    Some((current_board, solved_board))
}

fn map_args_to_strings(args: Vec<String>) -> (String, String) {
    let arg1 = args
        .get(1)
        .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardsMissing.get_message()))
        .to_string();
    let arg2 = args
        .get(2)
        .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardsMissing.get_message()))
        .to_string();
    (arg1, arg2)
}

fn map_string_to_tiles(input: String) -> Vec<Vec<Tile>> {
    let rows: Vec<String> = input.split('-').map(|s| s.to_string()).collect();
    rows.iter()
        .map(|row| row.split(' ').map(|s| Tile::new(s.to_string())).collect())
        .collect()
}

fn map_tiles_to_board(tiles: Vec<Vec<Tile>>) -> Board {
    Board::new(tiles).unwrap_or_else(|| panic!("{}", ErrorMessage::BoardIncorrect.get_message()))
}

fn valid_boards(current_board: &Board, solved_board: &Board) -> bool {
    let current_tiles = current_board.get_tiles();
    let solved_tiles = solved_board.get_tiles();
    let are_tiles_same = !current_tiles
        .iter()
        .any(|tile| !solved_tiles.contains(tile));
    let are_boards_equal = current_board.get_width() == solved_board.get_width()
        && current_board.get_height() == solved_board.get_height();
    are_tiles_same && are_boards_equal
}
