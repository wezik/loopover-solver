pub enum ErrorMessage {
    BoardsMissing,
    BoardsMismatched,
    BoardUnsolvable,
    BoardIncorrect,
}

impl ErrorMessage {
    pub fn get_message(&self) -> &str {
        match self {
            ErrorMessage::BoardsMissing => "One or both board arguments are missing. Please provide a starting board and a solved board.",
            ErrorMessage::BoardsMismatched => "The starting board and the solved board have different tiles. Please make sure they consist of the same tiles.",
            ErrorMessage::BoardUnsolvable => "The provided board is unsolvable. Please provide a solvable board.",
            ErrorMessage::BoardIncorrect => "The provided board is invalid. It must be at least 2x2 and have all rows of the same size."
        }
    }
}
