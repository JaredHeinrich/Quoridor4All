use crate::enums::Side;
use crate::vector_util::Vector;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Goal {
    is_x_coordinate: bool, //defines if goal_line sets the x or the y coordinate of the goal line
    goal_line: i16
}
impl Goal {
    //konstruktor
    pub fn new(player_side: &Side, board_size: i16) -> Self {
        match player_side {
            //wenn der Spieler unten startet, ist sein ziel nach oben zu kommen, also y-koordinate == 0
            Side::Bottom => {
                Self{
                    is_x_coordinate: false,
                    goal_line: 0,
                }
            },
            //...
            Side::Left => {
                Self{
                    is_x_coordinate: true,
                    goal_line: board_size - 1,
                }
            },
            //...
            Side::Top => {
                Self{
                    is_x_coordinate: false,
                    goal_line: board_size - 1,
                }
            },
            //...
            Side::Right => {
                Self{
                    is_x_coordinate: true,
                    goal_line: 0,
                }
            },
        }
    }

    //konstruktor für unit tests, um direkteren zugriff auf die werte der atribute zu haben.
    //wird nur bei ausführung der Tests kompiliert.
    #[cfg(test)]
    pub fn test_new(
        is_x_coordinate: bool,
        goal_line: i16
        ) -> Self {
        Self{
            is_x_coordinate,
            goal_line,
        }
    }

    //getter
    pub fn is_x_coordinate(&self) -> bool {
        self.is_x_coordinate
    }
    pub fn goal_line(&self) -> i16 {
        self.goal_line
    }
    //getter

    //gibt zurück, ob sich der Punkt eines Ortsvektors auf der Ziellinie befindet.
    pub fn is_in_goal_line(&self, position: Vector) -> bool {
        match self.is_x_coordinate {
            true => position.x() == self.goal_line(),
            false => position.y() == self.goal_line(),
        }
    }
    
}
