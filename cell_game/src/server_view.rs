use std::{
    iter::{repeat, Filter, FlatMap, Map, Repeat, Zip},
    slice::Iter,
};

use crate::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    game_view::GameView,
    player::Player,
    pos::Circle,
};

pub struct ServerView<'a> {
    players: &'a Vec<Player>,
    food: &'a Vec<FoodCell>,
    view_area: Option<Circle>,
}

impl<'a> ServerView<'a> {
    pub fn new(
        players: &'a Vec<Player>,
        food: &'a Vec<FoodCell>,
        view_area: Option<Circle>,
    ) -> Self {
        Self {
            players,
            food,
            view_area,
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

impl<'a> GameView<'a> for ServerView<'a> {
    type P = Map<
        Filter<
            Zip<
                FlatMap<
                    Iter<'a, Player>,
                    Iter<'a, PlayerCell>,
                    fn(&'a Player) -> Iter<'a, PlayerCell>,
                >,
                Repeat<Option<Circle>>,
            >,
            fn(&(&'a PlayerCell, Option<Circle>)) -> bool,
        >,
        fn((&'a PlayerCell, Option<Circle>)) -> &'a PlayerCell,
    >;
    type F = Map<
        Filter<
            Zip<Iter<'a, FoodCell>, Repeat<Option<Circle>>>,
            fn(&(&FoodCell, Option<Circle>)) -> bool,
        >,
        fn((&FoodCell, Option<Circle>)) -> &FoodCell,
    >;

    fn player_cells(&'a self) -> Self::P {
        fn fmap(p: &Player) -> Iter<'_, PlayerCell> {
            p.cells().iter()
        }

        self.players
            .iter()
            .flat_map(fmap as fn(&Player) -> Iter<'_, PlayerCell>)
            .zip(repeat(self.view_area))
            .filter(cell_visible as fn(&(&PlayerCell, Option<Circle>)) -> bool)
            .map(cell_from_tuple as fn((&PlayerCell, Option<Circle>)) -> &PlayerCell)
    }

    fn food_cells(&'a self) -> Self::F {
        self.food
            .iter()
            .zip(repeat(self.view_area))
            .filter(cell_visible as fn(&(&FoodCell, Option<Circle>)) -> bool)
            .map(cell_from_tuple as fn((&FoodCell, Option<Circle>)) -> &FoodCell)
    }

    fn view_area(&self) -> Option<Circle> {
        self.view_area
    }
}
