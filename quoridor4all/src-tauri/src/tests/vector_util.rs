use crate::{enums::{Color, Side}, structs::game::{Game, Player}, vector_util::{Vector, VectorUtil}};

fn init_game() -> Game {
    let players = [
        Player{
            player_name: "Player1".to_string(),
            pawn_color: Color::Red,
            pawn_side: Side::Bottom,
        },
        Player{
            player_name: "Player2".to_string(),
            pawn_color: Color::Green,
            pawn_side: Side::Left,
        },
        Player{
            player_name: "Player3".to_string(),
            pawn_color: Color::Blue,
            pawn_side: Side::Top,
        },
        Player{
            player_name: "Player4".to_string(),
            pawn_color: Color::Yellow,
            pawn_side: Side::Right,
        },
    ];
    Game::new(9,10,players)
}

#[test]
fn is_on_pawn_grid_positive_1() {
    let game = init_game();
    let vector = Vector::new(8,8);
    assert!(vector.is_on_pawn_grid(&game));
}

#[test]
fn is_on_pawn_grid_positive_2() {
    let game = init_game();
    let vector = Vector::new(0,0);
    assert!(vector.is_on_pawn_grid(&game));
}

#[test]
fn is_on_pawn_grid_negative_1() {
    let game = init_game();
    let vector = Vector::new(9,8);
    assert!(!vector.is_on_pawn_grid(&game));
}

#[test]
fn is_on_pawn_grid_negative_2() {
    let game = init_game();
    let vector = Vector::new(0,-1);
    assert!(!vector.is_on_pawn_grid(&game));
}

#[test]
fn is_on_wall_grid_positive_1() {
    let game = init_game();
    let vector = Vector::new(0,0);
    assert!(vector.is_on_wall_grid(&game));
}

#[test]
fn is_on_wall_grid_positive_2() {
    let game = init_game();
    let vector = Vector::new(7,7);
    assert!(vector.is_on_wall_grid(&game));
}

#[test]
fn is_on_wall_grid_negative_1() {
    let game = init_game();
    let vector = Vector::new(0,-1);
    assert!(!vector.is_on_wall_grid(&game));
}

#[test]
fn is_on_wall_grid_negative_2() {
    let game = init_game();
    let vector = Vector::new(8,7);
    assert!(!vector.is_on_wall_grid(&game));
}

#[test]
fn add_positive_1() {
    let v1 = Vector::new(0,0);
    let v2 = Vector::new(0,0);
    let exp_vector = Vector::new(0,0);
    let act_vector = v1.add(v2);
    assert_eq!(exp_vector, act_vector);
}

#[test]
fn add_positive_2() {
    let v1 = Vector::new(10,3);
    let v2 = Vector::new(-5,2);
    let exp_vector = Vector::new(5,5);
    let act_vector = v1.add(v2);
    assert_eq!(exp_vector, act_vector);
}

#[test]
fn add_negative_1() {
    let v1 = Vector::new(1,1);
    let v2 = Vector::new(2,4);
    let not_exp_vector = Vector::new(5,5);
    let act_vector = v1.add(v2);
    assert_ne!(not_exp_vector, act_vector);
}

#[test]
fn add_negative_2() {
    let v1 = Vector::new(4,2);
    let v2 = Vector::new(2,4);
    let not_exp_vector = Vector::new(3,3);
    let act_vector = v1.add(v2);
    assert_ne!(not_exp_vector, act_vector);
}

#[test]
fn subtract_positive_1() {
    let v1 = Vector::new(0,0);
    let v2 = Vector::new(0,0);
    let exp_vector = Vector::new(0,0);
    let act_vector = v1.subtract(v2);
    assert_eq!(exp_vector, act_vector);
}

#[test]
fn subtract_positive_2() {
    let v1 = Vector::new(12,6);
    let v2 = Vector::new(6,-6);
    let exp_vector = Vector::new(6,12);
    let act_vector = v1.subtract(v2);
    assert_eq!(exp_vector, act_vector);
}

#[test]
fn subtract_negative_1() {
    let v1 = Vector::new(0,0);
    let v2 = Vector::new(0,0);
    let not_exp_vector = Vector::new(1,1);
    let act_vector = v1.subtract(v2);
    assert_ne!(not_exp_vector, act_vector);
}

#[test]
fn subtract_negative_2() {
    let v1 = Vector::new(1,1);
    let v2 = Vector::new(1,-1);
    let not_exp_vector = Vector::new(2,2);
    let act_vector = v1.subtract(v2);
    assert_ne!(not_exp_vector, act_vector);
}

#[test]
fn revert_positive_1() {
    let vector = Vector::new(0,0);
    let exp_vector = Vector::new(0,0);
    let act_vector = vector.revert();
    assert_eq!(exp_vector, act_vector);
}

#[test]
fn revert_positive_2() {
    let vector = Vector::new(1,-1);
    let exp_vector = Vector::new(-1,1);
    let act_vector = vector.revert();
    assert_eq!(exp_vector, act_vector);
}

#[test]
fn revert_negative_1() {
    let vector = Vector::new(0,0);
    let not_exp_vector = Vector::new(1,0);
    let act_vector = vector.revert();
    assert_ne!(not_exp_vector, act_vector);
}

#[test]
fn revert_negative_2() {
    let vector = Vector::new(-1,-1);
    let not_exp_vector = Vector::new(1,0);
    let act_vector = vector.revert();
    assert_ne!(not_exp_vector, act_vector);
}

