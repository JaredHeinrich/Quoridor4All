//Prototyp implementation des Factory patterns. Das Factory Pattern trennt die Erzeugung von einem
//Object von diesem selbst. Es vereinfacht die Erzeugung von komplexen Objecten und könnte in
//unserem Spiel genutzt werden um verschiedene Game-Arten zu generieren, hier am Beispiel von
//einem Vier Spieler-Game und einem Zwei Spieler-Game umgesetzt, das Factory pattern erleichtert
//die erzeugung des komplexen Game Objectes. Ebenfalls ermöglicht es, dass erst zur Laufzeit
//entschieden werden muss, welche Game-Art genutzt werden soll
use game::{Factory, FourPlayerGameFactory, TwoPlayerGameFactory};
fn main() {
    let four_player_game = FourPlayerGameFactory::create();
    let two_player_game = TwoPlayerGameFactory::create();

    println!("four player game:");
    println!("{}", four_player_game.number_of_players());

    println!("two player game:");
    println!("{}", two_player_game.number_of_players());
}
pub mod game {

    //Factory Interface
    pub trait Factory {
        fn create() -> Box<dyn Game>;
    }

    //Factory Klasse für Vier Spieler
    pub struct FourPlayerGameFactory{}
    //implementation der Factory Klasse für Vier Spieler
    impl Factory for FourPlayerGameFactory{
        fn create() -> Box<dyn Game> {
            let number_of_available_walls: u32 = 10;
            let board_size: u32 = 9;

            //erstelle Spieler
            let p1: Player = Player::new(board_size, Position::Bottom, number_of_available_walls);
            let p2: Player = Player::new(board_size, Position::Left, number_of_available_walls);
            let p3: Player = Player::new(board_size, Position::Top, number_of_available_walls);
            let p4: Player = Player::new(board_size, Position::Right, number_of_available_walls);

            let players: [Player; 4] = [p1, p2, p3, p4];
            
            //leere Liste an Walls
            let walls: Vec<Wall> = Vec::new();

            //gibt Vier Spieler Game zurück
            Box::new(FourPlayerGame{
                players,
                walls,
                board_size,
            })
        }
    }

    pub struct TwoPlayerGameFactory{}
    impl Factory for TwoPlayerGameFactory{
        fn create() -> Box<dyn Game> {
            let number_of_available_walls: u32 = 10;
            let board_size: u32 = 9;

            let p1: Player = Player::new(board_size, Position::Bottom, number_of_available_walls);
            let p2: Player = Player::new(board_size, Position::Top, number_of_available_walls);

            let players: [Player; 2] = [p1, p2];
            
            let walls: Vec<Wall> = Vec::new();

            Box::new(TwoPlayerGame{
                players,
                walls,
                board_size,
            })
        }
    }
    //Game Interface wird von Factory zurückgegeben
    pub trait Game{
        fn number_of_players(&self) -> u16;
    }

    //Vier Spieler Variante
    pub struct FourPlayerGame{
        players: [Player;4],
        walls: Vec<Wall>,
        board_size: u32,
    } 
    impl Game for FourPlayerGame{
        fn number_of_players(&self) -> u16 {
            self.players.len() as u16
        }
    }

    pub struct TwoPlayerGame{
        players: [Player;2],
        walls: Vec<Wall>,
        board_size: u32,
    } 
    impl Game for TwoPlayerGame{
        fn number_of_players(&self) -> u16 {
            self.players.len() as u16
        }
    }


    //
    // Nicht mehr unbedingt Notwendig für das Factory Pattern
    // Klassen für Spieler Goal Wall und Enum für Start Position des Spielers
    // sowie implementation für konstruktoren, welche jedoch nur von der Factory und anderen
    // klassen dieses moduls genutzt werden können.
    //


    struct Player {
        pawn_coordinate: (u32,u32),
        goal: Goal,
        number_of_available_walls: u32,
    }

    impl Player {
        fn new(board_size: u32, position: Position, number_of_available_walls: u32) -> Self{
            let pawn_coordinate: (u32,u32) = Self::get_start_coordinate(board_size, &position);
            let goal: Goal = Goal::new(&position, board_size);
            Self{
                pawn_coordinate,
                goal,
                number_of_available_walls,
            }
        }
        fn get_start_coordinate(board_size: u32, position: &Position) -> (u32,u32) {
            let board_start: u32 = 0; //lowest index of board
            let board_end: u32 = board_size -1; //board_end is the highest index of the board
            let half_board: u32 = board_end/2; //half_board is the index at the half of the board | Example board_size = 9 => 0 1 2 3 4 5 6 7 8 => half_board = 4
            match position {
                Position::Top => (half_board,board_start),
                Position::Right => (board_end,half_board),
                Position::Bottom => (half_board,board_end),
                Position::Left => (board_start,half_board),
            }
        }
    }

    struct Goal {
        is_x_coordinate: bool, //if the goal is defined by x coordinate. if its false the goal is defined by y coordinate
        coordinate: u32,
    }

    impl Goal {
        fn new(position: &Position, board_size: u32) -> Self{
            match position {
                Position::Top => {
                    Self{
                        is_x_coordinate: false,
                        coordinate: board_size - 1,
                    }
                },
                Position::Right => {
                    Self{
                        is_x_coordinate: true,
                        coordinate: 0,
                    }
                },
                Position::Bottom => {
                    Self{
                        is_x_coordinate: false,
                        coordinate: 0,
                    }
                },
                Position::Left => {
                    Self{
                        is_x_coordinate: true,
                        coordinate: board_size - 1,
                    }
                },
            }
        }
    }

    struct Wall {
        is_vertical: bool, //if the wall is vertical. if its false the wall is horizontal
        coordinate: (u32,u32),
    }

    enum Position {
        Top,
        Right,
        Bottom,
        Left,
    }
}
