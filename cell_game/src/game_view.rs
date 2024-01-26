use crate::{
    cells::{food_cell::FoodCell, player_cell::PlayerCell},
    ids::PlayerId,
    player_info::PlayerInfo,
    pos::Circle,
};

pub trait GameView {
    fn player_cells(&self) -> impl Iterator<Item = PlayerCell>;
    fn food_cells(&self) -> impl Iterator<Item = FoodCell>;
    fn player_infos(&self) -> impl Iterator<Item = &PlayerInfo>;
    fn view_area(&self) -> Circle;
    /// The player who the view belongs to
    fn owner(&self) -> PlayerId;
}
