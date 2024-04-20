use crate::{structs::wall::Wall, vector_util::Vector};

#[test]
fn new_horizontal() {
    let position = Vector::new(5,5);
    let is_horizontal = true;
    let act_wall = Wall::new(position, is_horizontal);
    let exp_position = Vector::new(5,5);
    let exp_is_horizontal = true;
    assert_eq!(act_wall.position(), exp_position);
    assert_eq!(act_wall.is_horizontal(), exp_is_horizontal);
}

#[test]
fn new_vertical() {
    let position = Vector::new(4,6);
    let is_horizontal = false;
    let act_wall = Wall::new(position, is_horizontal);
    let exp_position = Vector::new(4,6);
    let exp_is_horizontal = false;
    assert_eq!(act_wall.position(), exp_position);
    assert_eq!(act_wall.is_horizontal(), exp_is_horizontal);
}

#[test]
fn is_in_conflict_with_positive_same_position_same_direction() {
    let wall_1 = Wall::new(Vector::new(5,5), true);
    let wall_2 = Wall::new(Vector::new(5,5), true);
    assert!(wall_1.is_in_conflict_with(&wall_2));
}

#[test]
fn is_in_conflict_with_positive_same_position_different_direction() {
    let wall_1 = Wall::new(Vector::new(5,5), true);
    let wall_2 = Wall::new(Vector::new(5,5), false);
    assert!(wall_1.is_in_conflict_with(&wall_2));
}

#[test]
fn is_in_conflict_with_positive_different_position_same_direction_horizontal_1() {
    let wall_1 = Wall::new(Vector::new(5,5), true);
    let wall_2 = Wall::new(Vector::new(6,5), true);
    assert!(wall_1.is_in_conflict_with(&wall_2));
}

#[test]
fn is_in_conflict_with_positive_different_position_same_direction_horizontal_2() {
    let wall_1 = Wall::new(Vector::new(5,5), true);
    let wall_2 = Wall::new(Vector::new(4,5), true);
    assert!(wall_1.is_in_conflict_with(&wall_2));
}

#[test]
fn is_in_conflict_with_positive_different_position_same_direction_vertical_1() {
    let wall_1 = Wall::new(Vector::new(5,5), false);
    let wall_2 = Wall::new(Vector::new(5,6), false);
    assert!(wall_1.is_in_conflict_with(&wall_2));
}

#[test]
fn is_in_conflict_with_positive_different_position_same_direction_vertical_2() {
    let wall_1 = Wall::new(Vector::new(5,5), false);
    let wall_2 = Wall::new(Vector::new(5,4), false);
    assert!(wall_1.is_in_conflict_with(&wall_2));
}

#[test]
fn is_in_conflict_with_negative_different_position_same_direction_1() {
    let wall_1 = Wall::new(Vector::new(5,5), false);
    let wall_2 = Wall::new(Vector::new(5,3), false);
    assert!(!wall_1.is_in_conflict_with(&wall_2));
}

#[test]
fn is_in_conflict_with_negative_different_position_same_direction_2() {
    let wall_1 = Wall::new(Vector::new(5,5), true);
    let wall_2 = Wall::new(Vector::new(7,5), true);
    assert!(!wall_1.is_in_conflict_with(&wall_2));
}

#[test]
fn is_in_conflict_with_negative_different_position_different_direction_1() {
    let wall_1 = Wall::new(Vector::new(5,5), true);
    let wall_2 = Wall::new(Vector::new(6,5), false);
    assert!(!wall_1.is_in_conflict_with(&wall_2));
}

#[test]
fn is_in_conflict_with_negative_different_position_different_direction_2() {
    let wall_1 = Wall::new(Vector::new(5,5), true);
    let wall_2 = Wall::new(Vector::new(5,6), false);
    assert!(!wall_1.is_in_conflict_with(&wall_2));
}

