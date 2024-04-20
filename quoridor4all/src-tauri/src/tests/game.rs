use crate::{enums::{Color::*, Side::*}, structs::{game::{CurrentPawn, Game, Player}, goal::Goal, history::GameHistory, pawn::Pawn, wall::Wall}, vector_util::Vector};

#[test]
fn get_valid_next_positions_empty() {
    let pawns = vec![
        Pawn::test_new(
            Vector::new(4,4),
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
    let act_moves = game.get_valid_next_positions();
    let exp_move_1 = Vector::new(4,3);
    let exp_move_2 = Vector::new(4,5);
    let exp_move_3 = Vector::new(3,4);
    let exp_move_4 = Vector::new(5,4);
    assert!(act_moves.contains(&exp_move_1));
    assert!(act_moves.contains(&exp_move_2));
    assert!(act_moves.contains(&exp_move_3));
    assert!(act_moves.contains(&exp_move_4));
}

#[test]
fn check_pawn_path_positive() {
        let pawns = vec![
            Pawn::test_new(
                Vector::new(4,7),
                10,
                Goal::test_new(false, 0),
                "Player 1".to_string(),
                Red
                )
        ];
        let walls = vec![
            Wall::new(Vector::new(0,4), true),
            Wall::new(Vector::new(2,4), true),
            Wall::new(Vector::new(4,4), true),
            Wall::new(Vector::new(6,4), true),
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        assert!(game.check_pawn_path(0));

}

#[test]
fn check_pawn_path_negative() {
        let pawns = vec![
            Pawn::test_new(
                Vector::new(4,7),
                10,
                Goal::test_new(false, 0),
                "Player 1".to_string(),
                Red
                )
        ];
        let walls = vec![
            Wall::new(Vector::new(0,4), true),
            Wall::new(Vector::new(2,4), true),
            Wall::new(Vector::new(4,4), true),
            Wall::new(Vector::new(6,4), true),
            Wall::new(Vector::new(7,5), true),
            Wall::new(Vector::new(6,5), false),
        ];
        let game = Game::test_new(
            pawns,
            CurrentPawn(0),
            GameHistory::new(),
            9,
            walls
            );
        assert!(!game.check_pawn_path(0));

}
