use std::mem;

pub struct Board {
    tiles: Vec<Vec<Tile>>,
    solved_state: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
}

impl Board {
    pub fn new(tiles: Vec<Vec<Tile>>, solved_state: Vec<Vec<Tile>>) -> Board {
        Board {
            height: tiles.len(),
            width: tiles.len(),
            tiles,
            solved_state,
        }
    }

    pub fn move_col_up(&mut self, x: usize, distance: usize) {
        for _ in 0..distance % self.height {
            let mut buffer = mem::take(&mut self.tiles[0][x]);
            for y in (1..self.height).rev() {
                let next_buffer = mem::replace(&mut self.tiles[y][x], buffer);
                buffer = next_buffer;
            }
            self.tiles[0][x] = buffer;
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
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Tile {
    value: String,
}

impl Tile {
    pub fn new(s: String) -> Tile {
        Tile { value: s }
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    fn test_tile(s: &str) -> Tile {
        Tile::new(s.to_string())
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

        let mut board = Board::new(starting.clone(), starting);

        // When
        board.move_col_up(1, expected.len() * 2 + 2);
        board.move_col_up(0, 1);

        // Then
        assert_eq!(format!("{:?}", board.tiles), format!("{:?}", expected));
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

        let mut board = Board::new(starting.clone(), starting);

        // When
        board.move_col_down(1, expected.len() * 4 + 1);
        board.move_col_down(2, 2);

        // Then
        assert_eq!(format!("{:?}", board.tiles), format!("{:?}", expected));
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

        let mut board = Board::new(starting.clone(), starting);

        // When
        board.move_row_right(0, expected.len() * 2 + 2);
        board.move_row_right(2, 1);

        // Then
        assert_eq!(format!("{:?}", board.tiles), format!("{:?}", expected));
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

        let mut board = Board::new(starting.clone(), starting);

        // When
        board.move_row_left(2, expected.len() * 5 + 2);
        board.move_row_left(1, 1);

        // Then
        assert_eq!(format!("{:?}", board.tiles), format!("{:?}", expected));
    }
}
