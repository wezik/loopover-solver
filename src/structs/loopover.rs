use std::mem;

#[derive(Clone, Debug)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
}

impl Board {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Option<Board> {
        if tiles.len() < 2 {
            let len = tiles[0].len();
            if len < 2 || tiles.iter().any(|row| row.len() != len) {
                return None;
            }
        }
        Some(Board {
            height: tiles.len(),
            width: tiles[0].len(),
            tiles,
        })
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
}

#[derive(Debug, Default, Clone)]
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
