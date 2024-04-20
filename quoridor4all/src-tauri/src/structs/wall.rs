use serde::{Deserialize, Serialize};

use crate::vector_util::{Vector, VectorUtil};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Wall {
    position: Vector,
    is_horizontal: bool, //wenn die wand nicht horizontal ist ist sie vertikal
}
impl Wall {
    //konstruktor
    pub fn new(position: Vector, is_horizontal: bool) -> Self {
        Self {
            position,
            is_horizontal,
        }
    }
    //getter
    pub fn position(&self) -> Vector {
        self.position
    }
    pub fn is_horizontal(&self) -> bool {
        self.is_horizontal
    }
    //getter
    //gibt den richtungsvektor der Wand zurück.
    fn directional_vector(&self) -> Vector {
        match self.is_horizontal {
            true => Vector::new(1,0),
            false => Vector::new(0,1),
        }
    }
    //überprüft, ob die Wand im konflikt mit einer anderen Wand steht.
    pub fn is_in_conflict_with(&self, wall: &Wall) -> bool {
        let is_parallel = self.is_horizontal == wall.is_horizontal;
        let pos_s = self.position;
        let pos_w = wall.position;
        let dv_s = self.directional_vector();
        //Wände dürfen sich nicht schneiden oder überlagern.
        if pos_s == pos_w {return true};
        if is_parallel {
            //wenn die Wände parallel sind, sind pos_a und pos_b die positionen, an denen ein Konflikt auftritt.
            //da die wände 2 einheiten lang sind, kann man sie nicht mit abstand 1 plazieren.
            let pos_a: Vector = pos_s.add(dv_s);
            if pos_a == pos_w {return true};
            let pos_b: Vector = pos_s.subtract(dv_s);
            if pos_b == pos_w {return true};
        };
        false
    }
}
