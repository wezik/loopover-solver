use std::env::Args;

use crate::board::{Board, Tile};

pub fn read_from_args(args: Args) -> (Board, Board) {
    let args: Vec<String> = args.collect();
    let starting = args.get(1).expect("missing argument");
    let solved = args.get(2).expect("missing solve argument");

    let starting_tiles = map_string_to_tiles(starting);
    let solved_tiles = map_string_to_tiles(solved);

    (Board::new(starting_tiles), Board::new(solved_tiles))
}

fn map_string_to_tiles(input: &str) -> Vec<Vec<Tile>> {
    let rows: Vec<String> = input.split('-').map(|s| s.to_string()).collect();
    rows.iter()
        .map(|row| row.split(' ').map(|s| Tile::new(s.to_string())).collect())
        .collect()
}
