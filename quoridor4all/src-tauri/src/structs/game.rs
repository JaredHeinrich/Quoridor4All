use std::i16;

use serde::{Deserialize, Serialize};

use super::history::{Move, PawnMove, WallMove};
use super::{pawn::Pawn, history::GameHistory, wall::Wall};
use crate::enums::{Color, Side, Direction};
use crate::vector_util::{VectorUtil, Vector};
use crate::NUMBER_OF_PLAYERS;

//Darstellung eines Spieler wie er im UI erstellt wird.
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
    //konstruktor
    pub fn new(
        board_size: i16,
        number_of_walls_per_player: i16,
        players: [Player; NUMBER_OF_PLAYERS],
    ) -> Self {
        //für jeden spieler wird ein pawn erstellt.
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
        //initialisierung von Walls, history und CurrentPawn
        Self {
            pawns,
            current_pawn,
            history,
            board_size,
            walls,
        }
    }

    //konstruktor für unit test für direkteren zugriff auf die werte der attribute.
    //wird nur für Tests kompiliert.
    #[cfg(test)]
    pub fn test_new(
        pawns: Vec<Pawn>,
        current_pawn: CurrentPawn,
        history: GameHistory,
        board_size: i16,
        walls: Vec<Wall>
        ) -> Self {
        Self{
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
  
    pub fn undo_last_move(&mut self) -> Result<(Vector, bool),String> {
    //Vector is either the new pawn position or the wall position. Bool indicates if the move was a
    //PawnMove or a WallMove
        let last_move_opt = self.history.pop_last_move();
        let last_move = match last_move_opt {
            Some(m) => m,
            None => return Err("no move to undo".to_string())
        };
        let result = match &last_move {
            Move::PawnMove(pm) => {
                let reverse_movement = pm.movement().revert();
                let prev_pawn = self.pawns.get_mut(self.current_pawn.get_prev()).unwrap();
                prev_pawn.move_pawn(reverse_movement);
                let new_pos = prev_pawn.position();
                (new_pos, true)
            },
            Move::WallMove(_) => {
                let wall = self.walls.pop();
                match wall {
                    None => return Err("no wall to remove".to_string()),
                    Some(w) => {
                        let wall_position = w.position();
                        (wall_position, false)
                    },
                }
            }
        };
        self.current_pawn.set_prev();
        Ok(result)

    }
    //bewegt die Figur des Aktuellen Spielers zu der neuen Position.
    //allowed_positions sind die Positionen, welche erlaubt wären. Diese müssen entweder aus dem
    //Puffer entnommen oder vorher neu berechnet werden.
    pub fn move_current_pawn(&mut self, new_position: Vector, allowed_positions: &Vec<Vector>) -> Result<(Vector, bool, String),String> {
        if !allowed_positions.contains(&new_position) {
            return Err("not a valid move".to_string());
        }
        let pawn = self.pawns.get_mut(self.current_pawn.get()).unwrap();
        let movement: Vector = new_position.subtract(pawn.position());
        pawn.move_pawn(movement);
        let new_pos = pawn.position();
        let did_win = pawn.goal().is_in_goal_line(new_pos);
        let player_name = pawn.player_name();
        self.current_pawn.set_next();
        self.history
            .add_move(Move::PawnMove(PawnMove::new(movement)));
        Ok((new_pos, did_win, player_name.to_string()))
    }

    //gibt alle möglichen Züge für die Figur des aktullen Spielers zurück.
    pub fn get_valid_next_positions(&self) -> Vec<Vector> {
        let mut res: Vec<Vector> = Vec::new();
        let pawn_pos = self.pawns.get(self.current_pawn.get()).unwrap().position();
        res.append(&mut self.check_step(&Direction::Up, pawn_pos, 0));
        res.append(&mut self.check_step(&Direction::Right, pawn_pos, 0));
        res.append(&mut self.check_step(&Direction::Down, pawn_pos, 0));
        res.append(&mut self.check_step(&Direction::Left, pawn_pos, 0));
        res
    }

    //gibt die Möglichen neuen positionen zurück, welche es für eine bewegung in die mitgegeben
    //richtung gibt.
    //pawn_position: gibt die Position an von der aus die Züge berechnet werden.
    //jumps: die Möglichen positionen werdne rekursiv berechnet. Es ist möglich, über eine andere
    //Spielfigur zu springen. Bei einer bestimmten anordnung der Figuren und wänden, kann es zu
    //einer endlosen rekursion kommen, daher wird abgebrochen, wenn versucht wird öfter zu
    //springen, als es Spieler gibt.
    fn check_step(
        &self,
        move_direction: &Direction,
        pawn_position: Vector,
        jumps: i16,
    ) -> Vec<Vector> {
        let mut res = Vec::new();
        if jumps >= NUMBER_OF_PLAYERS as i16 {
            return res;
        }
        let movement = move_direction.to_vector();
        let new_pos = pawn_position.add(movement);
        //wenn der Zug das Feld verlässt einen leeren vec zurückgeben.
        if !new_pos.is_on_pawn_grid(self) {
            return res;
        };
        //wenn der Zug durch eine Wand blockiert wird einen leeren vec zurückgeben.
        if self.does_wall_block_move(&move_direction, pawn_position) {
            return res;
        }
        //überprüfen, ob der Zug auf einer anderen Figur landet.
        let mut new_pos_on_pawn = false;
        for pawn in self.pawns.iter() {
            if pawn.position() == new_pos {
                new_pos_on_pawn = true;
                break;
            }
        }
        //wenn der Zug auf einer anderen Figur landet, das überspringen behandeln und die jump zahl
        //um 1 erhöhen.
        if new_pos_on_pawn {
            return self.check_jump(move_direction, new_pos, jumps + 1);
        }
        //wenn kein sonderfall auftritt einen vec mit der neuen position zurückgeben.
        res.push(new_pos);
        res
    }

    //behandelt sprünge.
    fn check_jump(&self, move_direction: &Direction, pawn_pos: Vector, jumps: i16) -> Vec<Vector> {
        let mut res = Vec::new();
        //versucht die Figur gerade zu überspringen.
        res.append(&mut self.check_step(move_direction, pawn_pos, jumps));
        //falls dass nicht klappt auch nach links und rechts versuchen.
        if res.len() == 0 {
            res.append(&mut self.check_step(&move_direction.turn_left(), pawn_pos, jumps));
            res.append(&mut self.check_step(&move_direction.turn_right(), pawn_pos, jumps));
        }
        res
    }

    //prüft ob eine wand erlaubt ist.
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
        //erzeugt eine kopie des Spiels und fügt dort die Wand ein.
        //dann wird überprüft, ob alle Spieler ihr Ziel noch erreichen können.
        let new_wall = new_wall.clone();
        let mut temp_game = self.clone();
        temp_game.walls.push(new_wall);
        if !temp_game.check_pawn_paths() {
            return false;
        }
        true
    }

    //plaziert eine Wand.
    pub fn place_wall(&mut self, new_wall: Wall) -> Result<(), String> {
        if !self.is_wall_valid(&new_wall) {
            return Err("wall is not valid".to_string());
        }
        //veringert die wände des aktullen spielers, plaziert die Wand, lässt den nächsten Spieler
        //spielen und fügt den Zug in die history hinzu.
        self.pawns.get_mut(self.current_pawn.get()).unwrap().dec_number_of_walls();
        self.walls.push(new_wall.clone());
        self.current_pawn.set_next();
        self.history.add_move(Move::WallMove(WallMove::new(new_wall.position())));
        Ok(())
    }

    //checks if all pawns can reach the according goal line
    pub fn check_pawn_paths(&self) -> bool {
        for pawn_index in 0..NUMBER_OF_PLAYERS {
            if !self.check_pawn_path(pawn_index) {
                return false;
            }
        }
        true
    }

    //überprüft, ob es für die durch den index gegebene Figur einen Pfad ans Ziel gibt.
    pub fn check_pawn_path(&self, pawn_index: usize) -> bool {
        let mut visited_positions = Vec::new();
        let mut current_position = self.pawns[pawn_index].position();

        //erste Position wurde bereits berücksichtigt
        visited_positions.push(current_position.clone());
        let mut index: usize = 0; //index um über visited_positions rüberzuiterieren und für jede vorhandene Position die angrenzenden validen Positionen hinzuzufügen

        //wenn für alle visited_positons alle Nachbarn überprüft und hinzugefügt worden sind, endet die while-Schleife -->alle möglichen Felder abgesucht
        while visited_positions.get(index).is_some() {
            //betrachte die aktuelle Position
            current_position = visited_positions[index].clone();

            //wenn aktuelle Position im Ziel liegt, wurde ein Pfad gefunden
            if self.pawns[pawn_index]
                .goal()
                .is_in_goal_line(current_position)
            {
                return true;
            }

            //Hole dir alle validen Nachbarpositionen
            let valid_positions = self.get_valid_next_positions_without_other(current_position);

            //Füge Nachbarpositionen hinzu, wenn diese nicht bereits in visited_positions enthalten sind
            for position in valid_positions {
                if !visited_positions.contains(&position) {
                    visited_positions.push(position);
                }
            }

            //nächste position in visited_positions auswählen
            index += 1;
        }
        false
    }

    //get_valid_neighbor_positions ohne beachtung der anderen Figuren
    fn get_valid_next_positions_without_other(&self, pawn_pos: Vector) -> Vec<Vector> {
        let directions = [
            &Direction::Up,
            &Direction::Right,
            &Direction::Down,
            &Direction::Left,
        ];

        directions
            .iter()
            .filter_map(|dir| self.check_step_without_other(dir, pawn_pos))
            .collect()
    }

    //selbes vorgehen wie bei check_step, jedoch ohne beachtung der anderen Figuren.
    fn check_step_without_other(
        &self,
        move_direction: &Direction,
        pawn_position: Vector,
    ) -> Option<Vector> {
        let movement = move_direction.to_vector();
        let new_pos = pawn_position.add(movement);
        if !new_pos.is_on_pawn_grid(self) {
            return None;
        };
        if self.does_wall_block_move(&move_direction, pawn_position) {
            return None;
        }
        Some(new_pos)
    }

    //überprüft, ob eine Bewegung durch einen Wand auf dem Feld blockiert wird.
    fn does_wall_block_move(&self, move_direction: &Direction, pawn_pos: Vector) -> bool {
        let mut blocking_walls: Vec<Wall> = Vec::new();
        match move_direction {
            //wenn die Bewegung nach oben gerichtet ist, sind pos_a, pos_b positionen, auf welchen
            //eine wagerechte Wand den Zug blockieren würde.
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
            //... senkrechte Wand ...
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
            //... wagerechte Wand ...
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
            //... senkrechte Wand ...
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
        //überprüfen, ob die Wände auf dem Feld eine Blockierende enthalten.
        for wall in self.walls.iter() {
            if blocking_walls.contains(&wall) {
                return true;
            }
        }
        false
    }
} // impl Game

#[derive(Clone)]
pub struct CurrentPawn(pub usize);
impl CurrentPawn{
    //wer ist gerade dran?
    fn get(&self) -> usize {
        self.0
    }
    //wer ist nachher dran?
    fn get_next(&self) -> usize {
        (self.0 + 1) % NUMBER_OF_PLAYERS
    }
    //wer war vorher dran?
    fn get_prev(&self) -> usize {
        (self.0 + NUMBER_OF_PLAYERS - 1) % NUMBER_OF_PLAYERS
    }
    //der Nächste ist dran.
    fn set_next(&mut self) {
        self.0 = (self.0 + 1) % NUMBER_OF_PLAYERS;
    }
    //der vorherige ist dran (z.B. bei undo_last_move).
    fn set_prev(&mut self) {
        self.0 = (self.0 + NUMBER_OF_PLAYERS - 1) % NUMBER_OF_PLAYERS;
    }
}

//Tests für private Methoden.
#[cfg(test)]
pub mod tests{
    use crate::{enums::{Direction::*, Color::*}, structs::{goal::Goal, history::GameHistory, pawn::Pawn, wall::Wall}, vector_util::Vector};

    use super::{CurrentPawn, Game};

    #[test]
    fn does_wall_block_move_positive_up_1 () {
        let pawns = vec![];
        let walls = vec![
            Wall::new(Vector::new(3,3), true),
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        assert!(game.does_wall_block_move(&Up, Vector::new(4,4)));
    }

    #[test]
    fn does_wall_block_move_positive_up_2 () {
        let pawns = vec![];
        let walls = vec![
            Wall::new(Vector::new(4,3), true),
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        assert!(game.does_wall_block_move(&Up, Vector::new(4,4)));
    }

    #[test]
    fn does_wall_block_move_positive_right_1 () {
        let pawns = vec![];
        let walls = vec![
            Wall::new(Vector::new(4,4), false),
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        assert!(game.does_wall_block_move(&Right, Vector::new(4,4)));
    }

    #[test]
    fn does_wall_block_move_positive_right_2 () {
        let pawns = vec![];
        let walls = vec![
            Wall::new(Vector::new(4,3), false),
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        assert!(game.does_wall_block_move(&Right, Vector::new(4,4)));
    }

    #[test]
    fn does_wall_block_move_positive_down_1 () {
        let pawns = vec![];
        let walls = vec![
            Wall::new(Vector::new(4,4), true),
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        assert!(game.does_wall_block_move(&Down, Vector::new(4,4)));
    }

    #[test]
    fn does_wall_block_move_positive_down_2 () {
        let pawns = vec![];
        let walls = vec![
            Wall::new(Vector::new(3,4), true),
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        assert!(game.does_wall_block_move(&Down, Vector::new(4,4)));
    }

    #[test]
    fn does_wall_block_move_positive_left_1 () {
        let pawns = vec![];
        let walls = vec![
            Wall::new(Vector::new(3,3), false),
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        assert!(game.does_wall_block_move(&Left, Vector::new(4,4)));
    }

    #[test]
    fn does_wall_block_move_positive_left_2 () {
        let pawns = vec![];
        let walls = vec![
            Wall::new(Vector::new(3,4), false),
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        assert!(game.does_wall_block_move(&Left, Vector::new(4,4)));
    }
    #[test]
    fn check_jump_empty() {
        let pawns = vec![];
        let walls = vec![];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        let act_moves = game.check_jump(&Up, Vector::new(4,4), 1);
        let exp_move = Vector::new(4,3);
        let not_exp_move = Vector::new(2,2);
        assert!(act_moves.contains(&exp_move));
        assert!(!act_moves.contains(&not_exp_move));
    }

    #[test]
    fn check_jump_blocked_by_border() {
        let pawns = vec![];
        let walls = vec![];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        let act_moves = game.check_jump(&Up, Vector::new(4,0), 1);
        let exp_move_1 = Vector::new(3,0);
        let exp_move_2 = Vector::new(5,0);
        let not_exp_move = Vector::new(2,2);
        assert!(act_moves.contains(&exp_move_1));
        assert!(act_moves.contains(&exp_move_2));
        assert!(!act_moves.contains(&not_exp_move));
    }

    #[test]
    fn check_jump_corner() {
        let pawns = vec![];
        let walls = vec![];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        let act_moves = game.check_jump(&Up, Vector::new(0,0), 1);
        let exp_move_1 = Vector::new(1,0);
        let not_exp_move = Vector::new(2,2);
        assert!(act_moves.contains(&exp_move_1));
        assert!(!act_moves.contains(&not_exp_move));
    }

    #[test]
    fn check_step_empty() {
        let pawns = vec![];
        let walls = vec![];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        let act_moves = game.check_step(&Up, Vector::new(4,4), 0);
        let exp_move = Vector::new(4,3);
        assert!(act_moves.contains(&exp_move));
    }

    #[test]
    fn check_step_on_pawn() {
        let pawns = vec![
            Pawn::test_new(
                Vector::new(4,3),
                10,
                Goal::test_new(false, 0),
                "Player 1".to_string(),
                Red
                )
        ];
        let walls = vec![];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        let act_moves = game.check_step(&Up, Vector::new(4,4), 0);
        let exp_move = Vector::new(4,2);
        assert!(act_moves.contains(&exp_move));
    }

    #[test]
    fn check_step_with_wall() {
        let pawns = vec![];
        let walls = vec![
            Wall::new(Vector::new(3,3), true)
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        let act_moves = game.check_step(&Up, Vector::new(4,4), 0);
        let not_exp_move = Vector::new(4,3);
        assert!(act_moves.len() == 0);
        assert!(!act_moves.contains(&not_exp_move));
    }

    #[test]
    fn check_step_max_jumps() {
        let pawns = vec![];
        let walls = vec![];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        let act_moves = game.check_step(&Up, Vector::new(4,4), 4);
        let not_exp_move = Vector::new(4,3);
        assert!(act_moves.len() == 0);
        assert!(!act_moves.contains(&not_exp_move));
    }

    #[test]
    fn check_step_without_other_empty() {
        let pawns = vec![];
        let walls = vec![];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        let act_move = game.check_step_without_other(&Up, Vector::new(4,4));
        let exp_move = Vector::new(4,3);
        let act_move = act_move.unwrap();
        assert_eq!(act_move, exp_move);
    }

    #[test]
    fn check_step_without_other_with_pawn() {
        let pawns = vec![
            Pawn::test_new(
                Vector::new(4,3),
                10,
                Goal::test_new(false, 0),
                "Player 1".to_string(),
                Red
                )
        ];
        let walls = vec![];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        let act_move = game.check_step_without_other(&Up, Vector::new(4,4));
        let exp_move = Vector::new(4,3);
        let act_move = act_move.unwrap();
        assert_eq!(act_move, exp_move);
    }

    #[test]
    fn check_step_without_other_with_wall() {
        let pawns = vec![];
        let walls = vec![
            Wall::new(Vector::new(3,3), true)
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        let act_move = game.check_step_without_other(&Up, Vector::new(4,4));
        let exp_move = None;
        assert_eq!(act_move, exp_move);
    }

    #[test]
    fn get_valid_next_positions_without_other_empty() {
        let pawns = vec![];
        let walls = vec![];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        let act_moves = game.get_valid_next_positions_without_other(Vector::new(4,4));
        let exp_move_1 = Vector::new(4,3);
        let exp_move_2 = Vector::new(4,5);
        let exp_move_3 = Vector::new(3,4);
        let exp_move_4 = Vector::new(5,4);
        assert!(act_moves.contains(&exp_move_1));
        assert!(act_moves.contains(&exp_move_2));
        assert!(act_moves.contains(&exp_move_3));
        assert!(act_moves.contains(&exp_move_4));
    }
}
