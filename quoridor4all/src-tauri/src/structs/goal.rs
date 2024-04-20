use crate::enums::Side;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Goal {
    is_x_coordinate: bool, //defines if goal_line sets the x or the y coordinate of the goal line
    goal_line: i16
}
impl Goal {
    //constructor: takes player_side and board_size and returns goal of the player
    pub fn new(player_side: &Side, board_size: i16) -> Self {
        match player_side {
            Side::Bottom => {
                Self{
                    is_x_coordinate: false,
                    goal_line: 0,
                }
            },
            Side::Left => {
                Self{
                    is_x_coordinate: true,
                    goal_line: board_size - 1,
                }
            },
            Side::Top => {
                Self{
                    is_x_coordinate: false,
                    goal_line: board_size - 1,
                }
            },
            Side::Right => {
                Self{
                    is_x_coordinate: true,
                    goal_line: 0,
                }
            },
        }
    }

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
}
