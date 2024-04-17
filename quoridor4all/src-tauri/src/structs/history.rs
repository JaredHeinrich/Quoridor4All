#[derive(Clone)]
pub enum Move {
    WallMove(WallMove),
    PawnMove(PawnMove),
}

#[derive(Clone)]
pub struct WallMove {
    position: (i16,i16)
}

#[derive(Clone)]
pub struct PawnMove {
    movement: (i16,i16),
}

#[derive(Clone)]
pub struct GameHistory {
    moves: Vec<Move>
}
impl GameHistory {
    pub fn new() -> Self {
        Self{
            moves: Vec::new()
        }
    }
    pub fn undo_last_move(&mut self) -> Option<Move> {
        self.moves.pop()
    }
    pub fn add_move(&mut self, player_move: Move) {
        self.moves.push(player_move)
    }
}
