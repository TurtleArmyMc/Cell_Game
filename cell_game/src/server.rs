use std::{
    iter::{Filter, Map, Repeat, Zip},
    slice::Iter,
};

use rand::prelude::*;

use crate::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    game_view::GameView,
    pos::{Circle, Point, Rect},
};

pub struct GameServer {
    players: Vec<PlayerCell>,
    food: Vec<FoodCell>,
    bounds: Rect,
}

impl GameServer {
    const GAME_BOUNDS: Rect = Rect {
        top_left: Point { x: 0.0, y: 0.0 },
        width: 1920.0,
        height: 1080.0,
    };
    const VIEW_RADIUS_MULTIPLIER: f64 = 30.0;

    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            food: Vec::new(),
            bounds: Self::GAME_BOUNDS,
        }
    }

    pub fn tick(&mut self) {
        for p in self.players.iter_mut() {
            p.move_player(self.bounds)
        }
    }

    pub fn spawn_player(&mut self) {
        self.players
            .push(PlayerCell::spawn_new(self.bounds.center()))
    }

    pub fn spawn_food(&mut self) {
        self.food.push(FoodCell::spawn_new(Point {
            x: self.bounds.min_x() + (self.bounds.width * random::<f64>()),
            y: self.bounds.min_y() + (self.bounds.height * random::<f64>()),
        }))
    }

    pub fn set_move_to(&mut self, dest: Point) {
        for p in self.players.iter_mut() {
            p.set_move_towards(dest)
        }
    }

    pub fn game_view(&self) -> ServerView<'_> {
        ServerView(self)
    }

    fn cell_overlaps_view_radius<T: Cell>(&self, cell: &T) -> bool {
        match self.player_view_radius() {
            Some(view_area) => cell.hitbox().overlaps_circle(view_area),
            None => false,
        }
    }

    fn player_view_radius(&self) -> Option<Circle> {
        self.players
            .first()
            .map(|p| p.hitbox().scale_centered(Self::VIEW_RADIUS_MULTIPLIER))
    }

    fn cell_visible<'a, T: Cell>(&(cell, game): &(&'a T, &'a GameServer)) -> bool {
        game.cell_overlaps_view_radius(cell)
    }
}

pub struct ServerView<'a>(&'a GameServer);

pub type ServerViewIterator<'a, T: Cell> = Map<
    Filter<Zip<Iter<'a, T>, Repeat<&'a GameServer>>, fn(&(&'a T, &'a GameServer)) -> bool>,
    fn((&'a T, &'a GameServer)) -> &'a T,
>;

fn cell_from_tuple<'a, T: Cell>((cell, _): (&'a T, &'a GameServer)) -> &'a T {
    cell
}

impl<'a> GameView<'a, ServerViewIterator<'a, PlayerCell>, ServerViewIterator<'a, FoodCell>>
    for ServerView<'a>
{
    fn player_cells(&self) -> ServerViewIterator<'a, PlayerCell> {
        // This would ideally be a .filter call with a closure, but that would
        // prevent the iterator type for GameView from being specified because
        // the closure would have an anonymous type that could not be referenced
        self.0
            .players
            .iter()
            .zip(std::iter::repeat(self.0))
            .filter(GameServer::cell_visible as fn(&(&'a PlayerCell, &'a GameServer)) -> bool)
            .map(cell_from_tuple as fn((&'a PlayerCell, &'a GameServer)) -> &'a PlayerCell)
    }

    fn food_cells(&self) -> ServerViewIterator<'a, FoodCell> {
        // Again, would ideally just be a .filter call
        self.0
            .food
            .iter()
            .zip(std::iter::repeat(self.0))
            .filter(GameServer::cell_visible as fn(&(&'a FoodCell, &'a GameServer)) -> bool)
            .map(cell_from_tuple as fn((&'a FoodCell, &'a GameServer)) -> &'a FoodCell)
    }

    fn view_area(&self) -> Option<Circle> {
        self.0.player_view_radius()
    }
}
