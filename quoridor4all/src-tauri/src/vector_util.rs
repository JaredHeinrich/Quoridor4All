use serde::{Deserialize, Serialize};

use crate::structs::game::Game;

//Darstellung eines Vektors.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
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
    //Konstruktor für den Vektor
    pub fn new(x: i16, y: i16) -> Self {
        Vector{
            x,
            y
        }
    }
    
    //Getter
    pub fn x(&self) -> i16 {
        self.x
    }

    pub fn y(&self) -> i16 {
        self.y
    }
    //Getter
}

//implementation der Vector Utility
impl VectorUtil for Vector {
    //gibt einen Boolean wert zurück, ob sich der Punkt, mit diesem Ortsvektor auf dem Gitter für
    //die Spielfiguren befindet.
    fn is_on_pawn_grid(&self, game: &Game) -> bool {
        let grid_size = game.board_size();
        self.x >= 0 && self.y >= 0 && self.x < grid_size && self.y < grid_size
    }
    //gibt einen Boolean wert zurück, ob sich der Punkt, mit diesem Ortsvektor auf dem Gitter für
    //die Wände befindet.
    fn is_on_wall_grid(&self, game: &Game) -> bool {
        let grid_size = game.board_size() - 1;
        self.x >= 0 && self.y >= 0 && self.x < grid_size && self.y < grid_size
    }
    //Addiert zwei Vektoren und gibt das Ergebniss zurück.
    fn add(&self, vector: Self) -> Self {
        Vector{
           x: self.x + vector.x,
           y: self.y + vector.y
        }
    }
    //Subtrahiert zwei Vektoren und gibt das Ergebniss zurück.
    fn subtract(&self, vector: Self) -> Self {
        Vector{
           x: self.x - vector.x,
           y: self.y - vector.y
        }
    }
    //Multipliziert einen Vektor mit -1
    fn revert(&self) -> Self {
        Vector{
            x: -self.x,
            y: -self.y
        }
    }
}
