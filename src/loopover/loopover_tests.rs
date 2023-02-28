use crate::loopover::structs::{Board, Tile};
use crate::ErrorMessage;

fn test_tile(s: &str) -> Tile {
    Tile::new(s.to_string())
}

#[test]
#[should_panic]
fn test_panic_with_incorrect_board() {
    // Given
    let tiles = vec![
        vec![test_tile("A"), test_tile("B"), test_tile("C")],
        vec![
            test_tile("D"),
            test_tile("E"),
            test_tile("F"),
            test_tile("G"),
        ],
        vec![test_tile("H"), test_tile("I"), test_tile("J")],
    ];
    // When & Then
    Board::new(tiles).expect("should panic");
}

#[test]
fn test_move_col_up() {
    // Given
    let starting = vec![
        vec![test_tile("A"), test_tile("B"), test_tile("C")],
        vec![test_tile("D"), test_tile("E"), test_tile("F")],
        vec![test_tile("G"), test_tile("H"), test_tile("I")],
    ];
    let expected = vec![
        vec![test_tile("D"), test_tile("H"), test_tile("C")],
        vec![test_tile("G"), test_tile("B"), test_tile("F")],
        vec![test_tile("A"), test_tile("E"), test_tile("I")],
    ];

    let mut board = Board::new(starting)
        .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardIncorrect.get_message()));

    // When
    board.move_col_up(1, expected.len() * 2 + 2);
    board.move_col_up(0, 1);

    // Then
    assert_eq!(
        format!("{:?}", board.get_tiles()),
        format!("{:?}", expected)
    );
}

#[test]
fn test_move_col_down() {
    // Given
    let starting = vec![
        vec![test_tile("A"), test_tile("B"), test_tile("C")],
        vec![test_tile("D"), test_tile("E"), test_tile("F")],
        vec![test_tile("G"), test_tile("H"), test_tile("I")],
    ];
    let expected = vec![
        vec![test_tile("A"), test_tile("H"), test_tile("F")],
        vec![test_tile("D"), test_tile("B"), test_tile("I")],
        vec![test_tile("G"), test_tile("E"), test_tile("C")],
    ];

    let mut board = Board::new(starting)
        .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardIncorrect.get_message()));

    // When
    board.move_col_down(1, expected.len() * 4 + 1);
    board.move_col_down(2, 2);

    // Then
    assert_eq!(
        format!("{:?}", board.get_tiles()),
        format!("{:?}", expected)
    );
}

#[test]
fn test_move_row_right() {
    // Given
    let starting = vec![
        vec![test_tile("A"), test_tile("B"), test_tile("C")],
        vec![test_tile("D"), test_tile("E"), test_tile("F")],
        vec![test_tile("G"), test_tile("H"), test_tile("I")],
    ];
    let expected = vec![
        vec![test_tile("B"), test_tile("C"), test_tile("A")],
        vec![test_tile("D"), test_tile("E"), test_tile("F")],
        vec![test_tile("I"), test_tile("G"), test_tile("H")],
    ];

    let mut board = Board::new(starting)
        .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardIncorrect.get_message()));

    // When
    board.move_row_right(0, expected.len() * 2 + 2);
    board.move_row_right(2, 1);

    // Then
    assert_eq!(
        format!("{:?}", board.get_tiles()),
        format!("{:?}", expected)
    );
}

#[test]
fn test_move_row_left() {
    // Given
    let starting = vec![
        vec![test_tile("A"), test_tile("B"), test_tile("C")],
        vec![test_tile("D"), test_tile("E"), test_tile("F")],
        vec![test_tile("G"), test_tile("H"), test_tile("I")],
    ];
    let expected = vec![
        vec![test_tile("A"), test_tile("B"), test_tile("C")],
        vec![test_tile("E"), test_tile("F"), test_tile("D")],
        vec![test_tile("I"), test_tile("G"), test_tile("H")],
    ];

    let mut board = Board::new(starting)
        .unwrap_or_else(|| panic!("{}", ErrorMessage::BoardIncorrect.get_message()));

    // When
    board.move_row_left(2, expected.len() * 5 + 2);
    board.move_row_left(1, 1);

    // Then
    assert_eq!(
        format!("{:?}", board.get_tiles()),
        format!("{:?}", expected)
    );
}
