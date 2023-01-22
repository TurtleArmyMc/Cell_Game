use std::{
    iter::{repeat, Filter, Map, Repeat, Zip},
    slice::Iter,
};

use crate::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    game_view::GameView,
    player_info::{PlayerId, PlayerInfo},
    pos::Circle,
};

pub struct ServerView<'a> {
    players: &'a Vec<PlayerCell>,
    food: &'a Vec<FoodCell>,
    player_infos: &'a Vec<PlayerInfo>,
    view_area: Option<Circle>,
    owner: PlayerId,
}

impl<'a> ServerView<'a> {
    pub fn new(
        players: &'a Vec<PlayerCell>,
        food: &'a Vec<FoodCell>,
        player_infos: &'a Vec<PlayerInfo>,
        view_area: Option<Circle>,
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

#[inline]
fn cell_from_tuple<'a, T: Cell>((cell, _): (&'a T, Option<Circle>)) -> &'a T {
    cell
}

#[inline]
fn cell_visible<T: Cell>(&(cell, view_area): &(&T, Option<Circle>)) -> bool {
    match view_area {
        Some(area) => cell.hitbox().overlaps_circle(area),
        None => false,
    }
}

type ServerViewIterator<'a, T> = Map<
    Filter<Zip<Iter<'a, T>, Repeat<Option<Circle>>>, fn(&(&T, Option<Circle>)) -> bool>,
    fn((&T, Option<Circle>)) -> &T,
>;

impl<'a> GameView<'a> for ServerView<'a> {
    type P = ServerViewIterator<'a, PlayerCell>;
    type F = ServerViewIterator<'a, FoodCell>;
    type I = Iter<'a, PlayerInfo>;

    fn player_cells(&'a self) -> Self::P {
        self.get_cell_iterator(self.players)
    }

    fn food_cells(&'a self) -> Self::F {
        self.get_cell_iterator(self.food)
    }

    fn player_infos(&'a self) -> Self::I {
        self.player_infos.iter()
    }

    fn view_area(&self) -> Option<Circle> {
        self.view_area
    }

    fn owner(&self) -> PlayerId {
        self.owner
    }
}

impl<'a> ServerView<'a> {
    #[inline]
    fn get_cell_iterator<T: Cell>(&'a self, cells: &'a Vec<T>) -> ServerViewIterator<'a, T> {
        // This would ideally be a .filter call with a closure, but that would
        // prevent the iterator type for GameView from being specified because
        // the closure would have an anonymous type that could not be referenced
        cells
            .iter()
            .zip(repeat(self.view_area))
            .filter(cell_visible as fn(&(&T, Option<Circle>)) -> bool)
            .map(cell_from_tuple as fn((&T, Option<Circle>)) -> &T)
    }
}
