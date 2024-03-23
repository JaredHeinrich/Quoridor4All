//Builder Pattern für die Wall. WallBuilder ist eine Beispiel Implementation, später könnte dieses
//Pattern für z.B. den Pawn genutzt werden, da dieser eine komplexe Struktur ist und mehrere
//ähnliche Instanzen von diesem gebaut werden müssen.
use wall::WallBuilder;
fn main() {
    let w1 = WallBuilder::new() 
        .set_position((7,7))
        .set_is_vertical(true)
        .build();

    let w2 = WallBuilder::new() 
        .set_is_vertical(true)
        .build();

    println!("{:?}", w1); // Some(Wall) da Wall gebaut wurde
    println!("{:?}", w2); // None da keine position gesetzt wurde und die Wall nicht gebaut werden
                          // konnte
}

pub mod wall{
    #[derive(Debug)] // für die Ausgabe der Wall
    pub struct Wall {
        position: (u16,u16),
        is_vertical: bool,
    }
    impl Wall {
        // getter werden nicht zwingend benötigt
        pub fn position(&self) -> (u16,u16) {
            self.position
        }
        pub fn is_vertical(&self) -> bool {
            self.is_vertical
        }
    }

    pub struct WallBuilder {
        //rust kennt kein null alle atribute müssen initialisiert werden, daher wird der enum
        //Option genutzt
        position: Option<(u16, u16)>,
        is_vertical: Option<bool>,
    }
    impl WallBuilder {
        pub fn new() -> WallBuilder {
            //bei einem neuen Builder sind die atribute standartmäßig auf None also nicht gesetzt
            WallBuilder{
                position: None,
                is_vertical: None
            }
        }
        // setzt die Position Some(position), da es sich um eine Variante des Enum handeln muss
        pub fn set_position(&mut self, position: (u16, u16)) -> &mut Self {
            self.position = Some(position);
            self
        }
        // setzt den bool is_vertical
        pub fn set_is_vertical(&mut self, is_vertical: bool) -> &mut Self {
            self.is_vertical = Some(is_vertical);
            self
        }
        //baut die Wall gibt None zurück, wenn die Wall nicht gebaut werden konnte und gibt
        //Some(Wall) zurück, wenn die Wall gebaut wurde
        pub fn build(&self) -> Option<Wall> {
            //testet ob position gesetzt wurde
            let position = match self.position {
                Some(p) => p,
                None => return None
            };
            //testet ob is_vertical gesetzt wurde
            let is_vertical = match self.is_vertical {
                Some(v) => v,
                None => return None
            };
            //wenn beide atribute gesetzt wurde wird die wall gebaut und zurückgegeben
            Some(Wall{
                position,
                is_vertical
            })
        }
    }
}
