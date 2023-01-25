use std::{
    iter::{repeat, Cloned, Filter, Map, Repeat, Zip},
    slice::Iter,
};

use crate::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    game_view::GameView,
    ids::PlayerId,
    player_info::PlayerInfo,
    pos::Circle,
};

pub struct ServerView<'a> {
    players: &'a Vec<PlayerCell>,
    food: &'a Vec<FoodCell>,
    player_infos: &'a Vec<PlayerInfo>,
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

#[inline]
fn cell_from_tuple<'a, T: Cell>((cell, _): (&'a T, Circle)) -> &'a T {
    cell
}

#[inline]
fn cell_visible<T: Cell>(&(cell, view_area): &(&T, Circle)) -> bool {
    cell.hitbox().overlaps_circle(view_area)
}

type ServerViewIterator<'a, T> = Cloned<
    Map<
        Filter<Zip<Iter<'a, T>, Repeat<Circle>>, fn(&(&T, Circle)) -> bool>,
        fn((&T, Circle)) -> &T,
    >,
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

    fn view_area(&self) -> Circle {
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
            .filter(cell_visible as fn(&(&T, Circle)) -> bool)
            .map(cell_from_tuple as fn((&T, Circle)) -> &T)
            .cloned()
    }
}
