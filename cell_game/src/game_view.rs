use crate::{
    cells::{food_cell::FoodCell, player_cell::PlayerCell},
    pos::Circle,
};

pub trait GameView<'a, P, F>
where
    P: Iterator<Item = &'a PlayerCell>,
    F: Iterator<Item = &'a FoodCell>,
{
    fn player_cells(&self) -> P;
    fn food_cells(&self) -> F;
    /// Returns None if the viewer has no cells remaining
    fn view_area(&self) -> Option<Circle>;
}
