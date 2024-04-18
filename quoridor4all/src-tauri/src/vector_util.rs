use serde::{Deserialize, Serialize};

use crate::structs::game::Game;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vector {
    x: i16,
    y: i16,
}

pub trait VectorUtil{
    fn is_on_pawn_grid(&self, game: &Game) -> bool;
    fn is_on_wall_grid(&self, game: &Game) -> bool;
    fn add(&self, vector: Self) -> Self;
    fn subtract(&self, vector: Self) -> Self;
    fn revert(&self) -> Self;
}

impl Vector {
    pub fn new(x: i16, y: i16) -> Self {
        Vector{
            x,
            y
        }
    }
}

impl VectorUtil for Vector {
    fn is_on_pawn_grid(&self, game: &Game) -> bool {
        let grid_size = game.board_size();
        self.x >= 0 && self.y >= 0 && self.x < grid_size && self.y < grid_size
    }
    fn is_on_wall_grid(&self, game: &Game) -> bool {
        let grid_size = game.board_size() - 1;
        self.x >= 0 && self.y >= 0 && self.x < grid_size && self.y < grid_size
    }
    fn add(&self, vector: Self) -> Self {
        Vector{
           x: self.x + vector.x,
           y: self.y + vector.y
        }
    }
    fn subtract(&self, vector: Self) -> Self {
        Vector{
           x: self.x - vector.x,
           y: self.y - vector.y
        }
    }
    fn revert(&self) -> Self {
        Vector{
            x: -self.x,
            y: -self.y
        }
    }
}
