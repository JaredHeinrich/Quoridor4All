use crate::{enums::{Color::Red, Side::*}, structs::{goal::Goal, pawn::Pawn}, vector_util::Vector};

#[test]
fn new_bottom() {
    let board_size = 9;
    let pawn_side = Bottom;
    let number_of_walls = 10;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    let exp_position = Vector::new(4,8);
    let exp_goal = Goal::new(&Bottom, board_size);
    let exp_number_of_available_walls = 10;
    let exp_player_name = "Player 1".to_string();
    assert_eq!(act_pawn.position(), exp_position);
    assert_eq!(act_pawn.goal().clone(), exp_goal);
    assert_eq!(act_pawn.number_of_available_walls(), exp_number_of_available_walls);
    assert_eq!(act_pawn.player_name(), exp_player_name);
}

#[test]
fn new_left() {
    let board_size = 9;
    let pawn_side = Left;
    let number_of_walls = 10;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    let exp_position = Vector::new(0,4);
    let exp_goal = Goal::new(&Left, board_size);
    let exp_number_of_available_walls = 10;
    let exp_player_name = "Player 1".to_string();
    assert_eq!(act_pawn.position(), exp_position);
    assert_eq!(act_pawn.goal().clone(), exp_goal);
    assert_eq!(act_pawn.number_of_available_walls(), exp_number_of_available_walls);
    assert_eq!(act_pawn.player_name(), exp_player_name);
}

#[test]
fn new_top() {
    let board_size = 9;
    let pawn_side = Top;
    let number_of_walls = 10;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    let exp_position = Vector::new(4,0);
    let exp_goal = Goal::new(&Top, board_size);
    let exp_number_of_available_walls = 10;
    let exp_player_name = "Player 1".to_string();
    assert_eq!(act_pawn.position(), exp_position);
    assert_eq!(act_pawn.goal().clone(), exp_goal);
    assert_eq!(act_pawn.number_of_available_walls(), exp_number_of_available_walls);
    assert_eq!(act_pawn.player_name(), exp_player_name);
}

#[test]
fn new_right() {
    let board_size = 9;
    let pawn_side = Right;
    let number_of_walls = 10;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    let exp_position = Vector::new(8,4);
    let exp_goal = Goal::new(&Right, board_size);
    let exp_number_of_available_walls = 10;
    let exp_player_name = "Player 1".to_string();
    assert_eq!(act_pawn.position(), exp_position);
    assert_eq!(act_pawn.goal().clone(), exp_goal);
    assert_eq!(act_pawn.number_of_available_walls(), exp_number_of_available_walls);
    assert_eq!(act_pawn.player_name(), exp_player_name);
}

#[test]
fn new_bottom_small() {
    let board_size = 5;
    let pawn_side = Bottom;
    let number_of_walls = 5;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    let exp_position = Vector::new(2,4);
    let exp_goal = Goal::new(&Bottom, board_size);
    let exp_number_of_available_walls = 5;
    let exp_player_name = "Player 1".to_string();
    assert_eq!(act_pawn.position(), exp_position);
    assert_eq!(act_pawn.goal().clone(), exp_goal);
    assert_eq!(act_pawn.number_of_available_walls(), exp_number_of_available_walls);
    assert_eq!(act_pawn.player_name(), exp_player_name);
}

#[test]
fn new_left_small() {
    let board_size = 5;
    let pawn_side = Left;
    let number_of_walls = 5;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    let exp_position = Vector::new(0,2);
    let exp_goal = Goal::new(&Left, board_size);
    let exp_number_of_available_walls = 5;
    let exp_player_name = "Player 1".to_string();
    assert_eq!(act_pawn.position(), exp_position);
    assert_eq!(act_pawn.goal().clone(), exp_goal);
    assert_eq!(act_pawn.number_of_available_walls(), exp_number_of_available_walls);
    assert_eq!(act_pawn.player_name(), exp_player_name);
}

#[test]
fn new_top_small() {
    let board_size = 5;
    let pawn_side = Top;
    let number_of_walls = 5;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    let exp_position = Vector::new(2,0);
    let exp_goal = Goal::new(&Top, board_size);
    let exp_number_of_available_walls = 5;
    let exp_player_name = "Player 1".to_string();
    assert_eq!(act_pawn.position(), exp_position);
    assert_eq!(act_pawn.goal().clone(), exp_goal);
    assert_eq!(act_pawn.number_of_available_walls(), exp_number_of_available_walls);
    assert_eq!(act_pawn.player_name(), exp_player_name);
}

#[test]
fn new_right_small() {
    let board_size = 5;
    let pawn_side = Right;
    let number_of_walls = 5;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    let exp_position = Vector::new(4,2);
    let exp_goal = Goal::new(&Right, board_size);
    let exp_number_of_available_walls = 5;
    let exp_player_name = "Player 1".to_string();
    assert_eq!(act_pawn.position(), exp_position);
    assert_eq!(act_pawn.goal().clone(), exp_goal);
    assert_eq!(act_pawn.number_of_available_walls(), exp_number_of_available_walls);
    assert_eq!(act_pawn.player_name(), exp_player_name);
}

#[test]
fn move_pawn_1() {
    let board_size = 5;
    let pawn_side = Right;
    let number_of_walls = 5;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let mut act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    let exp_start_position = Vector::new(4,2);
    assert_eq!(act_pawn.position(), exp_start_position);
    act_pawn.move_pawn(Vector::new(-4,-2));
    let exp_end_position = Vector::new(0,0);
    assert_eq!(act_pawn.position(), exp_end_position);
}

#[test]
fn move_pawn_2() {
    let board_size = 5;
    let pawn_side = Right;
    let number_of_walls = 5;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let mut act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    let exp_start_position = Vector::new(4,2);
    assert_eq!(act_pawn.position(), exp_start_position);
    act_pawn.move_pawn(Vector::new(0,0));
    let exp_end_position = Vector::new(4,2);
    assert_eq!(act_pawn.position(), exp_end_position);
}

#[test]
fn inc_number_of_walls() {
    let board_size = 5;
    let pawn_side = Right;
    let number_of_walls = 5;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let mut act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    act_pawn.inc_number_of_walls();
    let exp_number_of_walls = 6;
    assert_eq!(act_pawn.number_of_available_walls(), exp_number_of_walls);
}

#[test]
fn dec_number_of_walls() {
    let board_size = 5;
    let pawn_side = Right;
    let number_of_walls = 5;
    let player_name = "Player 1".to_string();
    let pawn_color = Red;
    let mut act_pawn = Pawn::new( 
                             board_size,
                             pawn_side,
                             number_of_walls,
                             player_name,
                             pawn_color
                             );
    act_pawn.dec_number_of_walls();
    let exp_number_of_walls = 4;
    assert_eq!(act_pawn.number_of_available_walls(), exp_number_of_walls);
}
