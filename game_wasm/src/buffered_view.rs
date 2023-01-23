use std::{iter::Cloned, slice::Iter};

use cell_game::{
    cells::{food_cell::FoodCell, player_cell::PlayerCell},
    game_view::GameView,
    ids::PlayerId,
    player_info::PlayerInfo,
    pos::Circle,
};

pub struct BufferedView {
    players: Vec<PlayerCell>,
    food: Vec<FoodCell>,
    info: Vec<PlayerInfo>,
    view_area: Option<Circle>,
    owner: PlayerId,
}

impl BufferedView {
    pub fn new<'a, V: GameView<'a>>(view: &'a V) -> Self {
        Self {
            players: view.player_cells().collect(),
            food: view.food_cells().collect(),
            info: view.player_infos().cloned().collect(),
            view_area: view.view_area(),
            owner: view.owner(),
        }
    }
}

impl<'a> GameView<'a> for BufferedView {
    type P = Cloned<Iter<'a, PlayerCell>>;
    type F = Cloned<Iter<'a, FoodCell>>;
    type I = Iter<'a, PlayerInfo>;

    fn player_cells(&'a self) -> Self::P {
        self.players.iter().cloned()
    }

    fn food_cells(&'a self) -> Self::F {
        self.food.iter().cloned()
    }

    fn player_infos(&'a self) -> Self::I {
        self.info.iter()
    }

    fn view_area(&self) -> Option<Circle> {
        self.view_area
    }

    fn owner(&self) -> PlayerId {
        self.owner
    }
}
