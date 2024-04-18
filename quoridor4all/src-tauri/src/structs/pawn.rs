use crate::enums::{Side, Color};
use crate::structs::goal::Goal;
use crate::vector_util::{VectorUtil, Vector};

#[derive(Clone)]
pub struct Pawn {
    position: Vector,
    number_of_available_walls: i16,
    goal: Goal,
    player_name: String,
    pawn_color: Color,
}
impl Pawn {
    //constructor
    pub fn new(board_size: i16,
               pawn_side: Side,
               number_of_available_walls: i16,
               player_name: String,
               pawn_color: Color
               ) -> Self{
        let position: Vector = Self::get_start_coordinate(board_size, &pawn_side);
        let goal: Goal = Goal::new(&pawn_side, board_size);
        Self{
            position,
            goal,
            number_of_available_walls,
            player_name,
            pawn_color,
        }
    }
    fn get_start_coordinate(board_size: i16, pawn_side: &Side) -> Vector {
        let board_start: i16 = 0; //lowest index of board
        let board_end: i16 = board_size -1; //board_end is the highest index of the board
        let half_board: i16 = board_end/2; //half_board is the index at the half of the board | Example board_size = 9 => 0 1 2 3 4 5 6 7 8 => half_board = 4
        match pawn_side {
            Side::Bottom => Vector::new(half_board,board_end),
            Side::Left => Vector::new(board_start,half_board),
            Side::Top => Vector::new(half_board,board_start),
            Side::Right => Vector::new(board_end,half_board),
        }
    }
    //getter
    pub fn position(&self) -> Vector {
        self.position
    }
    pub fn number_of_available_walls(&self) -> i16 {
        self.number_of_available_walls
    }
    pub fn goal(&self) -> &Goal {
        &self.goal
    }
    pub fn player_name(&self) -> &str {
        &self.player_name
    }
    //getter
    pub fn move_pawn(&mut self, movement: Vector) {
        self.position = self.position.add(movement);
    }
    pub fn inc_number_of_walls(&mut self) {
        let number_of_walls = &mut self.number_of_available_walls;
        *number_of_walls = *number_of_walls + 1;
    }
    pub fn dec_number_of_walls(&mut self) {
        let number_of_walls = &mut self.number_of_available_walls;
        *number_of_walls = *number_of_walls - 1;
    }
}
