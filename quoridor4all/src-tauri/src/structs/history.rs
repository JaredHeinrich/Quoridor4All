use crate::vector_util::Vector;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Move {
    WallMove(WallMove),
    PawnMove(PawnMove),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WallMove {
    position: Vector
}

impl WallMove {
    pub fn new(position: Vector) -> Self {
        Self{
            position
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PawnMove {
    movement: Vector,
}

impl PawnMove {
    pub fn new(movement: Vector) -> Self {
        Self{
            movement
        }
    }
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
