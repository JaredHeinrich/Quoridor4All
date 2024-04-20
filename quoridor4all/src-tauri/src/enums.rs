use serde::{Deserialize, Serialize};
use crate::vector_util::Vector;

#[derive(Clone, Serialize, Deserialize)]
pub enum Side {
    Bottom,
    Left,
    Top,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    pub fn to_vector(&self) -> Vector {
        match self {
            Self::Up => Vector::new(0,-1),
            Self::Right => Vector::new(1,0),
            Self::Down => Vector::new(0,1),
            Self::Left => Vector::new(-1,0),
        }
    }
    pub fn revert(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
    pub fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
    pub fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Color {
    Yellow,
    Orange,
    Red,
    Purple,
    Blue,
    Green,
    Lime,
    Brown,
    Black,
}
