use crate::vector_util::Vector;
#[derive(Clone)]
pub enum Move {
    WallMove(WallMove),
    PawnMove(PawnMove),
}

#[derive(Clone)]
pub struct WallMove {
    position: Vector
}

impl WallMove {
    pub fn new(position: Vector) -> Self {
        Self{
            position
        }
    }
    pub fn position(&self) -> Vector {
        self.position
    }
}

#[derive(Clone)]
pub struct PawnMove {
    movement: Vector,
}

impl PawnMove {
    pub fn new(movement: Vector) -> Self {
        Self{
            movement
        }
    }
    pub fn movement(&self) -> Vector {
        self.movement
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
    pub fn pop_last_move(&mut self) -> Option<Move> {
        self.moves.pop()
    }
    pub fn add_move(&mut self, player_move: Move) {
        self.moves.push(player_move)
    }
}
