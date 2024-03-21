use game::{Factory, FourPlayerGameFactory, TwoPlayerGameFactory};
fn main() {
    let fpgf = FourPlayerGameFactory{};
    let tpgf = TwoPlayerGameFactory{};
    let fpg = fpgf.create();
    let tpg = tpgf.create();
}
pub mod game {

    pub trait Factory {
        fn create(&self) -> Box<dyn Game>;
    }

    pub struct FourPlayerGameFactory{}
    impl Factory for FourPlayerGameFactory{
        fn create(&self) -> Box<dyn Game> {
            let number_of_available_walls: u32 = 10;
            let board_size: u32 = 9;

            let p1: Player = Player::new(board_size, Position::Bottom, number_of_available_walls);
            let p2: Player = Player::new(board_size, Position::Left, number_of_available_walls);
            let p3: Player = Player::new(board_size, Position::Top, number_of_available_walls);
            let p4: Player = Player::new(board_size, Position::Right, number_of_available_walls);

            let players: [Player; 4] = [p1, p2, p3, p4];
            
            let walls: Vec<Wall> = Vec::new();

            Box::new(FourPlayerGame{
                players,
                walls,
                board_size,
            })
        }
    }

    pub struct TwoPlayerGameFactory{}
    impl Factory for TwoPlayerGameFactory{
        fn create(&self) -> Box<dyn Game> {
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
    pub trait Game{}

    #[derive(Debug)]
    pub struct FourPlayerGame{
        players: [Player;4],
        walls: Vec<Wall>,
        board_size: u32,
    } 
    impl Game for FourPlayerGame{}

    #[derive(Debug)]
    pub struct TwoPlayerGame{
        players: [Player;2],
        walls: Vec<Wall>,
        board_size: u32,
    } 
    impl Game for TwoPlayerGame{}

    #[derive(Debug)]
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

    #[derive(Debug)]
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

    #[derive(Debug)]
    struct Wall {
        is_vertical: bool, //if the wall is vertical. if its false the wall is horizontal
        coordinate: (u32,u32),
    }

    #[derive(Debug)]
    enum Position {
        Top,
        Right,
        Bottom,
        Left,
    }
}
