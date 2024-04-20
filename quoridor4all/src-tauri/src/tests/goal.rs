use crate::{enums::Side::*, structs::goal::Goal};

#[test]
fn new_bottom() {
    let player_side = Bottom;
    let board_size = 9;
    let act_goal = Goal::new(&player_side, board_size);
    let exp_goal_line = 0;
    let exp_is_x_coordinate = false;
    assert_eq!(act_goal.goal_line(), exp_goal_line);
    assert_eq!(act_goal.is_x_coordinate(), exp_is_x_coordinate);
}

#[test]
fn new_left() {
    let player_side = Left;
    let board_size = 9;
    let act_goal = Goal::new(&player_side, board_size);
    let exp_goal_line = 8;
    let exp_is_x_coordinate = true;
    assert_eq!(act_goal.goal_line(), exp_goal_line);
    assert_eq!(act_goal.is_x_coordinate(), exp_is_x_coordinate);
}

#[test]
fn new_top() {
    let player_side = Top;
    let board_size = 9;
    let act_goal = Goal::new(&player_side, board_size);
    let exp_goal_line = 8;
    let exp_is_x_coordinate = false;
    assert_eq!(act_goal.goal_line(), exp_goal_line);
    assert_eq!(act_goal.is_x_coordinate(), exp_is_x_coordinate);
}

#[test]
fn new_right() {
    let player_side = Right;
    let board_size = 9;
    let act_goal = Goal::new(&player_side, board_size);
    let exp_goal_line = 0;
    let exp_is_x_coordinate = true;
    assert_eq!(act_goal.goal_line(), exp_goal_line);
    assert_eq!(act_goal.is_x_coordinate(), exp_is_x_coordinate);
}
