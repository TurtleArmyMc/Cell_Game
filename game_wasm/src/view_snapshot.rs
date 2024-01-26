use cell_game::{
    cells::{food_cell::FoodCell, player_cell::PlayerCell},
    game_view::GameView,
    ids::PlayerId,
    player_info::PlayerInfo,
    pos::Circle,
};

/// Stores a view's current state so that it can be used later
pub struct ViewSnapshot {
    players: Vec<PlayerCell>,
    food: Vec<FoodCell>,
    info: Vec<PlayerInfo>,
    view_area: Circle,
    owner: PlayerId,
}

impl ViewSnapshot {
    pub fn new(view: &impl GameView) -> Self {
        Self {
            players: view.player_cells().collect(),
            food: view.food_cells().collect(),
            info: view.player_infos().cloned().collect(),
            view_area: view.view_area(),
            owner: view.owner(),
        }
    }
}

impl GameView for ViewSnapshot {
    fn player_cells(&self) -> impl Iterator<Item = PlayerCell> {
        self.players.iter().cloned()
    }

    fn food_cells(&self) -> impl Iterator<Item = FoodCell> {
        self.food.iter().cloned()
    }

    fn player_infos(&self) -> impl Iterator<Item = &PlayerInfo> {
        self.info.iter()
    }

    fn view_area(&self) -> Circle {
        self.view_area
    }

    fn owner(&self) -> PlayerId {
        self.owner
    }
}
