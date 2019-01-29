#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0...7, 0...7) => Some(ChessPosition(rank, file)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        return Self(position);
    }

    #[rustfmt::skip]
    pub fn can_attack(&self, other: &Queen) -> bool {
        let Queen(ChessPosition(x_0, y_0)) = *self;
        let Queen(ChessPosition(x_1, y_1)) = *other;
        if x_0 == x_1 { return true; } // same rank
        if y_0 == y_1 { return true; } // same file
        if (x_1 - x_0).abs() == (y_1 - y_0).abs(){ return true; } // same diagonal
        return false;
    }
}
