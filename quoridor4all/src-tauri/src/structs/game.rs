use std::i16;

use serde::{Deserialize, Serialize};

use super::history::{Move, PawnMove};
use super::{history::GameHistory, pawn::Pawn, wall::Wall};
use crate::enums::{Color, Direction, Side};
use crate::vector_util::{Vector, VectorUtil};
use crate::NUMBER_OF_PLAYERS;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub player_name: String,
    pub pawn_color: Color,
    pub pawn_side: Side,
}

#[derive(Clone)]
pub struct Game {
    pawns: Vec<Pawn>,
    current_pawn: CurrentPawn,
    history: GameHistory,
    board_size: i16,
    walls: Vec<Wall>,
}

impl Game {
    pub fn new(
        board_size: i16,
        number_of_walls_per_player: i16,
        players: [Player; NUMBER_OF_PLAYERS],
    ) -> Self {
        let pawns: Vec<Pawn> = players
            .into_iter()
            .map(|player| {
                Pawn::new(
                    board_size,
                    player.pawn_side,
                    number_of_walls_per_player,
                    player.player_name,
                    player.pawn_color,
                )
            })
            .collect();
        let current_pawn: CurrentPawn = CurrentPawn(0);
        let history: GameHistory = GameHistory::new();
        let walls: Vec<Wall> = Vec::new();
        Self {
            pawns,
            current_pawn,
            history,
            board_size,
            walls,
        }
    }

    pub fn board_size(&self) -> i16 {
        self.board_size
    }
    pub fn move_current_pawn(
        &mut self,
        new_position: Vector,
        allowed_positions: &Vec<Vector>,
    ) -> Result<Vector, String> {
        if !allowed_positions.contains(&new_position) {
            return Err("not a valid move".to_string());
        }
        let pawn = self.pawns.get_mut(self.current_pawn.get()).unwrap();
        let movement: Vector = new_position.subtract(pawn.position());
        pawn.move_pawn(movement);
        let new_pos = pawn.position();
        self.current_pawn.set_next();
        self.history
            .add_move(Move::PawnMove(PawnMove::new(movement)));
        Ok(new_pos)
    }

    pub fn get_valid_next_positions(&self) -> Vec<Vector> {
        let mut res: Vec<Vector> = Vec::new();
        let pawn_pos = self.pawns.get(self.current_pawn.get()).unwrap().position();
        res.append(&mut self.check_step(&Direction::Up, pawn_pos, 0));
        res.append(&mut self.check_step(&Direction::Right, pawn_pos, 0));
        res.append(&mut self.check_step(&Direction::Down, pawn_pos, 0));
        res.append(&mut self.check_step(&Direction::Left, pawn_pos, 0));
        res
    }

    fn check_step(
        &self,
        move_direction: &Direction,
        pawn_position: Vector,
        jumps: i16,
    ) -> Vec<Vector> {
        let mut res = Vec::new();
        if jumps > NUMBER_OF_PLAYERS as i16 - 1 {
            return res;
        }
        let movement = move_direction.to_vector();
        let new_pos = pawn_position.add(movement);
        if !new_pos.is_on_pawn_grid(self) {
            return res;
        };
        if self.does_wall_block_move(&move_direction, pawn_position) {
            return res;
        }
        let mut new_pos_on_pawn = false;
        for pawn in self.pawns.iter() {
            if pawn.position() == new_pos {
                new_pos_on_pawn = true;
                break;
            }
        }
        if new_pos_on_pawn {
            return self.check_jump(move_direction, new_pos, jumps + 1);
        }
        res.push(new_pos);
        res
    }

    fn check_jump(&self, move_direction: &Direction, pawn_pos: Vector, jumps: i16) -> Vec<Vector> {
        let mut res = Vec::new();
        res.append(&mut self.check_step(move_direction, pawn_pos, jumps));
        if res.len() == 0 {
            res.append(&mut self.check_step(&move_direction.turn_left(), pawn_pos, jumps));
            res.append(&mut self.check_step(&move_direction.turn_right(), pawn_pos, jumps));
        }
        res
    }

    pub fn is_wall_valid(&self, new_wall: &Wall) -> bool {
        let new_wall_pos = new_wall.position();
        if !new_wall_pos.is_on_wall_grid(self) {
            return false;
        };
        for wall in self.walls.iter() {
            if new_wall.is_in_conflict_with(wall) {
                return false;
            }
        }
        let new_wall = new_wall.clone();
        let mut temp_game = self.clone();
        temp_game.walls.push(new_wall);
        if !temp_game.check_pawn_paths() {
            return false;
        }
        true
    }

    pub fn place_wall(&mut self, new_wall: Wall) -> Result<(), String> {
        if !self.is_wall_valid(&new_wall) {
            return Err("wall is not valid".to_string());
        }
        self.pawns
            .get_mut(self.current_pawn.get())
            .unwrap()
            .dec_number_of_walls();
        self.walls.push(new_wall);
        self.current_pawn.set_next();
        Ok(())
    }

    pub fn check_pawn_paths(&self) -> bool {
        for pawn_index in 0..NUMBER_OF_PLAYERS {
            if !self.check_pawn_path(pawn_index) {
                return false;
            }
        }
        true
    }

    pub fn check_pawn_path(&self, pawn_index: usize) -> bool {
        let mut visited_positions = Vec::new();
        let mut current_position = self.pawns[pawn_index].position();

        //first position is already visited
        visited_positions.push(current_position.clone());
        let mut index: usize = 0;   //index to go over all visited_positions and add possible possitions for all positions in visited_positions

        //if for all visited_positions all possible neighbor positions are checked
        while visited_positions.get(index).is_some() {
            current_position = visited_positions[index].clone();

            //current position is a goal -->path found
            if self.pawns[pawn_index].goal().is_in_goal_line(&current_position) {
                return true;
            }

            //get neighbour positions which are valid
            let valid_positions = self.get_valid_next_positions_without_other(&current_position, pawn_index);

            //add valid_positions to visited_positions if not already in visited_positions
            for position in valid_positions {
                if !visited_positions.contains(&position) {
                    visited_positions.push(position);
                }
            }

            //next position in visited_positions
            index += 1;
        }
        false
    }

    //get_valid_neighbor_positions
    fn get_valid_next_positions_without_other(&self, pawn_index: usize) -> Vec<Vector> {
        let mut res: Vec<Vector> = Vec::new();
        let pawn_pos = self.pawns.get(pawn_index).unwrap().position();

        res.push(&mut self.check_step_without_other(&Direction::Up, pawn_pos));
        res.push(&mut self.check_step_without_other(&Direction::Right, pawn_pos));
        res.push(&mut self.check_step_without_other(&Direction::Down, pawn_pos));
        res.push(&mut self.check_step_without_other(&Direction::Left, pawn_pos));
        res
    }

    fn check_step_without_other(
        &self,
        move_direction: &Direction,
        pawn_position: Vector,
    ) -> Vector {
        let movement = move_direction.to_vector();
        let new_pos = pawn_position.add(movement);
        if !new_pos.is_on_pawn_grid(self) {
            return;
        };
        if self.does_wall_block_move(&move_direction, pawn_position) {
            return;
        }
        new_pos
    }

    fn does_wall_block_move(&self, move_direction: &Direction, pawn_pos: Vector) -> bool {
        let mut blocking_walls: Vec<Wall> = Vec::new();
        match move_direction {
            Direction::Up => {
                let pos_a = pawn_pos.add(Vector::new(-1, -1));
                let pos_b = pawn_pos.add(Vector::new(0, -1));
                if pos_a.is_on_wall_grid(self) {
                    blocking_walls.push(Wall::new(pos_a, true))
                }
                if pos_b.is_on_wall_grid(self) {
                    blocking_walls.push(Wall::new(pos_b, true))
                }
            }
            Direction::Right => {
                let pos_a = pawn_pos.add(Vector::new(0, -1));
                let pos_b = pawn_pos;
                if pos_a.is_on_wall_grid(self) {
                    blocking_walls.push(Wall::new(pos_a, false))
                }
                if pos_b.is_on_wall_grid(self) {
                    blocking_walls.push(Wall::new(pos_b, false))
                }
            }
            Direction::Down => {
                let pos_a = pawn_pos.add(Vector::new(-1, 0));
                let pos_b = pawn_pos;
                if pos_a.is_on_wall_grid(self) {
                    blocking_walls.push(Wall::new(pos_a, true))
                }
                if pos_b.is_on_wall_grid(self) {
                    blocking_walls.push(Wall::new(pos_b, true))
                }
            }
            Direction::Left => {
                let pos_a = pawn_pos.add(Vector::new(-1, -1));
                let pos_b = pawn_pos.add(Vector::new(-1, 0));
                if pos_a.is_on_wall_grid(self) {
                    blocking_walls.push(Wall::new(pos_a, false))
                }
                if pos_b.is_on_wall_grid(self) {
                    blocking_walls.push(Wall::new(pos_b, false))
                }
            }
        }
        for wall in self.walls.iter() {
            if blocking_walls.contains(&wall) {
                return true;
            }
        }
        false
    }
} // impl Game

#[derive(Clone)]
struct CurrentPawn(usize);
impl CurrentPawn {
    fn get(&self) -> usize {
        self.0
    }
    fn set_next(&mut self) {
        self.0 = (self.0 + 1) % NUMBER_OF_PLAYERS;
    }
    fn set_prev(&mut self) {
        self.0 = (self.0 + NUMBER_OF_PLAYERS - 1) % NUMBER_OF_PLAYERS;
    }
}
