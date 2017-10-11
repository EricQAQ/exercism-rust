pub type ChessPositionResult = Result<ChessPosition, ()>;

pub struct ChessPosition {
    x: i8,
    y: i8,
}

impl ChessPosition {
    pub fn new(x: i8, y: i8) -> ChessPositionResult {
        match (x >= 0 && x < 8, y >= 0 && y < 8) {
            (true, true) => Ok(ChessPosition { x: x, y: y }),
            _ => Err(()),
        }
    }
}

pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position: position }
    }

    pub fn can_attack(&self, target: &Queen) -> bool {
        return self.position.x == target.position.x
            || self.position.y == target.position.y
            || (self.position.x - target.position.x).abs()
                == (self.position.y - target.position.y).abs()
    }
}
