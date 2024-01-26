use crate::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    game_view::GameView,
    ids::PlayerId,
    player_info::PlayerInfo,
    pos::Circle,
};

pub struct ServerView<'a> {
    players: &'a [PlayerCell],
    food: &'a [FoodCell],
    player_infos: &'a [PlayerInfo],
    view_area: Circle,
    owner: PlayerId,
}

impl<'a> ServerView<'a> {
    pub fn new(
        players: &'a Vec<PlayerCell>,
        food: &'a Vec<FoodCell>,
        player_infos: &'a Vec<PlayerInfo>,
        view_area: Circle,
        owner: PlayerId,
    ) -> Self {
        Self {
            players,
            food,
            player_infos,
            view_area,
            owner,
        }
    }
}

impl GameView for ServerView<'_> {
    fn player_cells(&self) -> impl Iterator<Item = PlayerCell> {
        self.filter_visible_cells(self.players).cloned()
    }

    fn food_cells(&self) -> impl Iterator<Item = FoodCell> {
        self.filter_visible_cells(self.food).cloned()
    }

    fn player_infos(&self) -> impl Iterator<Item = &PlayerInfo> {
        self.player_infos.iter()
    }

    fn view_area(&self) -> Circle {
        self.view_area
    }

    fn owner(&self) -> PlayerId {
        self.owner
    }
}

impl<'a> ServerView<'a> {
    #[inline]
    fn filter_visible_cells<T: Cell>(&'a self, cells: &'a [T]) -> impl Iterator<Item = &'a T> {
        cells
            .iter()
            .filter(|cell| cell.hitbox().overlaps_circle(self.view_area))
    }
}
