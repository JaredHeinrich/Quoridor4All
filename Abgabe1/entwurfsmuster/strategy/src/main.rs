//Prototyp Implementation des Strategy Patterns. Das Strategy Pattern ermöglicht es einem
//verschiedene Implementationen, um die gleiche Funktionalität zu erfüllen, auszutauschen.
//In unserem Spiel ist es notwendig, zu überprüfen, ob jede Spielfigur einen Weg auf die Ziel-
//seite besitzt. Für die Lösung dieses Problems sind mehrere Algorithmen denkbar. Daher
//ist es wahrscheinlich, dass ein PathChecker in Zukunft ausgetauscht wird, wenn ein effizienterer
//oder einfacherer Algorithmus verwendet werden soll. Daher ist hier ein optimaler Anwendungsfall
//des Strategy Patterns gegeben. Der PathChecker prüft für einen Pawn die Erreichbarkeit der
//anderen Seite. Wie dies jedoch genau geschieht kann in unterschiedlichen Strategieen
//implementiert werden.
use path_finder::{Game, PathChecker1, PathChecker2, Pawn};

fn main() {
    //erstelle Pawns
    let pawns1: Vec<Pawn> = vec![
        Pawn::new((4,5)),
        Pawn::new((2,3)),
        Pawn::new((2,6)),
        Pawn::new((1,2)),
    ];
    let pawns2: Vec<Pawn> = vec![
        Pawn::new((6,6)),
        Pawn::new((5,7)),
        Pawn::new((8,4)),
        Pawn::new((8,5)),
    ];

    //erstelle path_checker
    let pc1: PathChecker1 = PathChecker1{};
    let pc2: PathChecker2 = PathChecker2{};

    //erstelle games
    let game1: Game<PathChecker1> = Game::new(pawns1, pc1);
    let game2: Game<PathChecker2> = Game::new(pawns2, pc2);

    //führe die check_paths funktion von den games aus, und gibt die ergebniss aus
    println!("game1:");
    println!("{}", game1.check_paths());

    println!("game2:");
    println!("{}", game2.check_paths());
}

mod path_finder{
    use std::u16;

    //PathChecker Interface
    pub trait PathChecker{
        fn check_path(&self, position: (u16, u16)) -> bool;
    }

    //erste Variante des PathCheckers
    pub struct PathChecker1 {}
    impl PathChecker for PathChecker1 {
        fn check_path(&self, position: (u16, u16)) -> bool {
            // demo path_checking algorithm
            position.0 + position.1 < 10
        }
    }

    //zweite Variante des PathCheckers
    pub struct PathChecker2 {}
    impl PathChecker for PathChecker2 {
        fn check_path(&self, position: (u16, u16)) -> bool {
            // demo path_checking algorithm
            position.0 + position.1 == 12
        }
    }

    //spielfiguren
    pub struct Pawn {
        position: (u16, u16)
    }
    impl Pawn {
        pub fn new(position: (u16, u16)) -> Self {
            Self{
                position
            }
        }
    }
    //Spiel mit generic T welches PathChecker implementiert
    pub struct Game<T: PathChecker> {
        pawns: Vec<Pawn>,
        //path_checker vom typ T
        path_checker: T
    }
    // implementation für eine Game mit generic T welches PathChecker implementiert
    impl<T: PathChecker> Game<T> {
        pub fn new(pawns: Vec<Pawn>, path_checker: T) -> Self{
            Self{
                pawns,
                path_checker
            }
        }
        //überprüft ob alle Pawns einen Weg haben wenn nein gibt es false zurück wenn ja dann true
        pub fn check_paths(&self) -> bool {
            for pawn in self.pawns.iter().enumerate() {
                let pawn_pos = pawn.1.position;
                let pawn_id = pawn.0;
                let has_path: bool = self.path_checker.check_path(pawn_pos);
                if !has_path {
                println!("Pawn: {} an Position: {:?} hat keinen Weg auf die andere Seite", pawn_id, pawn_pos);
                    return false;
                };
                println!("Pawn: {} an Position: {:?} hat einen Weg auf die andere Seite", pawn_id, pawn_pos);
            }
            true
        }
    }
}
