pub struct Board {
    tiles: Vec<Vec<Tile>>,
}

impl Board {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Board {
        Board { tiles }
    }
}

#[derive(Debug)]
pub struct Tile {
    value: String,
}

impl Tile {
    pub fn new(s: String) -> Tile {
        Tile { value: s }
    }
}
