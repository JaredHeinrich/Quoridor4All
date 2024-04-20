use crate::{structs::history::{GameHistory, Move, PawnMove, WallMove}, vector_util::Vector};

#[test]
fn history_add_and_undo() {
    let mut history = GameHistory::new();
    let pushed_value_1 = Move::WallMove(WallMove::new(Vector::new(1,2)));
    history.add_move(pushed_value_1.clone());
    let poped_value_1 = history.pop_last_move().unwrap();
    assert_eq!(pushed_value_1, poped_value_1);
}

#[test]
fn history_pop_if_empty() {
    let mut history = GameHistory::new();
    let poped_value_1: Option<Move> = history.pop_last_move();
    assert_eq!(poped_value_1, None);
}

#[test]
fn history_check_if_undo_deletes() {
    let mut history = GameHistory::new();
    let pushed_value_1 = Move::WallMove(WallMove::new(Vector::new(1,2)));
    history.add_move(pushed_value_1.clone());
    let poped_value_1 = history.pop_last_move().unwrap();
    assert_eq!(pushed_value_1, poped_value_1);
    let poped_value_2 = history.pop_last_move();
    assert_eq!(poped_value_2, None);
}

#[test]
fn history_multiple_adds_and_undos() {
    let mut history = GameHistory::new();
    let pushed_value_1 = Move::WallMove(WallMove::new(Vector::new(1,2)));
    history.add_move(pushed_value_1.clone());
    let pushed_value_2 = Move::PawnMove(PawnMove::new(Vector::new(5,7)));
    history.add_move(pushed_value_2.clone());

    let poped_value_1 = history.pop_last_move().unwrap();
    assert_eq!(pushed_value_2, poped_value_1);
    let poped_value_2 = history.pop_last_move().unwrap();
    assert_eq!(pushed_value_1, poped_value_2);
    let poped_value_3 = history.pop_last_move();
    assert_eq!(poped_value_3, None);
}
