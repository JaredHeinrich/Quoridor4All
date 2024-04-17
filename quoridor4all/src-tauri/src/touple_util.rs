use crate::structs::game::Game;

pub trait ToupleUtil{
    fn is_on_pawn_grid(&self, game: &Game) -> bool;
    fn is_on_wall_grid(&self, game: &Game) -> bool;
    fn add(&self, vector: Self) -> Self;
    fn subtract(&self, vector: Self) -> Self;
    fn revert(&self) -> Self;
}
impl ToupleUtil for (i16,i16) {
    fn is_on_pawn_grid(&self, game: &Game) -> bool {
        let grid_size = game.board_size();
        self.0 >= 0 && self.1 >= 0 && self.0 < grid_size && self.1 < grid_size

    }
    fn is_on_wall_grid(&self, game: &Game) -> bool {
        let grid_size = game.board_size() - 1;
        self.0 >= 0 && self.1 >= 0 && self.0 < grid_size && self.1 < grid_size
    }
    fn add(&self, vector: Self) -> Self {
        (self.0 + vector.0, self.1 + vector.1)
    }
    fn subtract(&self, vector: Self) -> Self {
        (self.0 - vector.0, self.1 - vector.1)
    }
    fn revert(&self) -> Self {
        (-self.0, -self.1)
    }
}
