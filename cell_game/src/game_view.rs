use crate::{
    cells::{food_cell::FoodCell, player_cell::PlayerCell},
    ids::PlayerId,
    player_info::PlayerInfo,
    pos::Circle,
};

pub trait GameView<'a> {
    type P: Iterator<Item = &'a PlayerCell>;
    type F: Iterator<Item = &'a FoodCell>;
    type I: Iterator<Item = &'a PlayerInfo>;

    fn player_cells(&'a self) -> Self::P;
    fn food_cells(&'a self) -> Self::F;
    fn player_infos(&'a self) -> Self::I;
    /// Returns None if the viewer has no cells remaining
    fn view_area(&self) -> Option<Circle>;
    /// The player who the view belongs to
    fn owner(&self) -> PlayerId;
}
