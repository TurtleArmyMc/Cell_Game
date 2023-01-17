use crate::{
    cells::{food_cell::FoodCell, player_cell::PlayerCell},
    pos::Circle,
};

pub trait GameView<'a> {
    type P: Iterator<Item = &'a PlayerCell>;
    type F: Iterator<Item = &'a FoodCell>;

    fn player_cells(&'a self) -> Self::P;
    fn food_cells(&'a self) -> Self::F;
    /// Returns None if the viewer has no cells remaining
    fn view_area(&self) -> Option<Circle>;
}
