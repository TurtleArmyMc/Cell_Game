use std::{
    iter::{Filter, Map, Repeat, Zip},
    slice::Iter,
};

use rand::prelude::*;

use crate::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    client_connection::{ClientConnection, PlayerInput},
    game_view::GameView,
    pos::{Circle, Point, Rect},
};

pub struct GameServer {
    players: Vec<PlayerCell>,
    food: Vec<FoodCell>,
    bounds: Rect,

    connections: Vec<Box<dyn for<'a> ClientConnection<'a, V = ServerView<'a>>>>,
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
            connections: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        for p in self.players.iter_mut() {
            p.move_player(self.bounds)
        }

        for conn in self.connections.iter_mut() {
            let input = conn.on_tick(ServerView {
                players: &self.players,
                food: &self.food,
                view_area: Self::player_view_radius(&self.players),
            });

            if let Some(PlayerInput { move_to }) = input {
                Self::set_move_to(&mut self.players, move_to)
            }
        }
    }

    pub fn add_connection(
        &mut self,
        conn: Box<dyn for<'a> ClientConnection<'a, V = ServerView<'a>>>,
    ) {
        self.connections.push(conn)
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

    fn set_move_to(players: &mut Vec<PlayerCell>, dest: Point) {
        for p in players.iter_mut() {
            p.move_towards_point(dest)
        }
    }

    fn player_view_radius(players: &Vec<PlayerCell>) -> Option<Circle> {
        players
            .first()
            .map(|p| p.hitbox().scale_centered(Self::VIEW_RADIUS_MULTIPLIER))
    }
}

pub struct ServerView<'a> {
    players: &'a Vec<PlayerCell>,
    food: &'a Vec<FoodCell>,
    view_area: Option<Circle>,
}

pub type ServerViewIterator<'a, T: Cell> = Map<
    Filter<Zip<Iter<'a, T>, Repeat<Option<Circle>>>, fn(&(&T, Option<Circle>)) -> bool>,
    fn((&T, Option<Circle>)) -> &T,
>;

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
    type P = ServerViewIterator<'a, PlayerCell>;
    type F = ServerViewIterator<'a, FoodCell>;

    fn player_cells(&'a self) -> Self::P {
        self.get_cell_iterator(self.players)
    }

    fn food_cells(&'a self) -> Self::F {
        self.get_cell_iterator(self.food)
    }

    fn view_area(&self) -> Option<Circle> {
        self.view_area
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
            .zip(std::iter::repeat(self.view_area))
            .filter(cell_visible as fn(&(&T, Option<Circle>)) -> bool)
            .map(cell_from_tuple as fn((&T, Option<Circle>)) -> &T)
    }
}
