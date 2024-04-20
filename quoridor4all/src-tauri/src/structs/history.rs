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

//Zug in dem eine Wand gesetzt wurde
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

//Zug in dem eine Figur bewegt wurde.
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
