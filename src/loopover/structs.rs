use std::{fmt, mem};

#[derive(Clone)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
    moves: Vec<String>,
}

impl Board {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Option<Board> {
        if tiles.len() >= 2 {
            let len = tiles[0].len();
            if len >= 2 && !tiles.iter().any(|row| row.len() != len) {
                return Some(Board {
                    height: tiles.len(),
                    width: tiles[0].len(),
                    tiles,
                    moves: Vec::new(),
                });
            }
        }
        None
    }

    pub fn move_col_up(&mut self, x: usize, distance: usize) {
        for _ in 0..distance % self.height {
            let mut buffer = mem::take(&mut self.tiles[0][x]);
            for y in (1..self.height).rev() {
                let next_buffer = mem::replace(&mut self.tiles[y][x], buffer);
                buffer = next_buffer;
            }
            self.tiles[0][x] = buffer;
            self.moves.push(format!("U{x}"));
        }
    }

    pub fn move_col_down(&mut self, x: usize, distance: usize) {
        for _ in 0..distance % self.height {
            let mut buffer = mem::take(&mut self.tiles[self.height - 1][x]);
            for y in 0..self.height - 1 {
                let next_buffer = mem::replace(&mut self.tiles[y][x], buffer);
                buffer = next_buffer;
            }
            self.tiles[self.height - 1][x] = buffer;
            self.moves.push(format!("D{x}"));
        }
    }

    pub fn move_row_left(&mut self, y: usize, distance: usize) {
        for _ in 0..distance % self.width {
            let mut buffer = mem::take(&mut self.tiles[y][0]);
            for x in (1..self.height).rev() {
                let next_buffer = mem::replace(&mut self.tiles[y][x], buffer);
                buffer = next_buffer;
            }
            self.tiles[y][0] = buffer;
            self.moves.push(format!("L{y}"));
        }
    }

    pub fn move_row_right(&mut self, y: usize, distance: usize) {
        for _ in 0..distance % self.width {
            let mut buffer = mem::take(&mut self.tiles[y][self.width - 1]);
            for x in 0..self.width - 1 {
                let next_buffer = mem::replace(&mut self.tiles[y][x], buffer);
                buffer = next_buffer;
            }
            self.tiles[y][self.width - 1] = buffer;
            self.moves.push(format!("R{y}"));
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y][x]
    }

    pub fn find_tile(&self, tile: &Tile) -> Option<(usize, usize)> {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, t) in row.iter().enumerate() {
                if t.get_value() == tile.get_value() {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn get_moves(&self) -> &Vec<String> {
        &self.moves
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in &self.tiles {
            s += &*format!("{:?}\n", row);
        }
        write!(f, "{}", s)
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct Tile {
    value: String,
}

impl Tile {
    pub fn new(s: String) -> Tile {
        Tile { value: s }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}]", self.value)
    }
}

#[cfg(test)]
mod tests {
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
    #[should_panic]
    fn test_panic_with_incorrect_board2() {
        // Given
        let tiles = vec![
            vec![test_tile("A"), test_tile("B"), test_tile("C")],
            vec![test_tile("D"), test_tile("E")],
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
}
