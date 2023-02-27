use crate::ErrorMessage;
use crate::modules::loopover::{Board, Tile};

pub fn map_args_to_board(arg1: String, arg2: String) -> (Board, Board) {
    let starting_tiles = map_string_to_tiles(arg1);
    let solved_tiles = map_string_to_tiles(arg2);

    let starting_board = Board::new(starting_tiles)
        .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardIncorrect.get_message()));
    let solved_board = Board::new(solved_tiles)
        .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardIncorrect.get_message()));

    (starting_board, solved_board)
}

fn map_string_to_tiles(input: String) -> Vec<Vec<Tile>> {
    let rows: Vec<String> = input.split('-').map(|s| s.to_string()).collect();
    rows.iter()
        .map(|row| row.split(' ').map(|s| Tile::new(s.to_string())).collect())
        .collect()
}
