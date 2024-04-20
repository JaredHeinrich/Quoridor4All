use crate::{enums::{Color::*, Side::*}, structs::game::{Game, Player}};

fn init_game() -> Game {
    let board_size = 9;
    let number_of_walls_per_player = 10;
    let players = [
        Player{
            player_name: "Player 1".to_string(),
            pawn_color: Red,
            pawn_side: Bottom,
        },
        Player{
            player_name: "Player 2".to_string(),
            pawn_color: Blue,
            pawn_side: Left,
        },
        Player{
            player_name: "Player 3".to_string(),
            pawn_color: Green,
            pawn_side: Top,
        },
        Player{
            player_name: "Player 4".to_string(),
            pawn_color: Yellow,
            pawn_side: Right,
        }
    ];
    Game::new(board_size,
              number_of_walls_per_player,
              players)
}

#[test]
fn new() {
    let act_game = init_game();
    let exp_board_size: i16 = 9;
    assert_eq!(act_game.board_size(), exp_board_size);
}
