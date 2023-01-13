use std::slice::Iter;

use crate::{
    cells::{food_cell::FoodCell, player_cell::PlayerCell},
    pos::{Point, Rect},
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
        self.food.push(FoodCell::spawn_new(self.bounds.center()))
    }

    pub fn set_move_to(&mut self, dest: Point) {
        for p in self.players.iter_mut() {
            p.set_move_towards(dest)
        }
    }

    pub fn player_cells(&self) -> Iter<'_, PlayerCell> {
        self.players.iter()
    }

    pub fn food_cells(&self) -> Iter<'_, FoodCell> {
        self.food.iter()
    }
}
