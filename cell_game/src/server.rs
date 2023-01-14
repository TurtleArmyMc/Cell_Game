use std::slice::Iter;

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
    const VIEW_RADIUS_MULTIPLIER: f64 = 7.0;

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

    pub fn game_view(&self) -> ServerView<'_> {
        ServerView(self)
    }

    fn player_view_radius(&self) -> Option<Circle> {
        self.players.first().map(|p| Circle {
            center: p.pos(),
            radius: p.radius() * Self::VIEW_RADIUS_MULTIPLIER,
        })
    }
}

pub struct ServerView<'a>(&'a GameServer);

impl<'a> GameView<'a, Iter<'a, PlayerCell>, Iter<'a, FoodCell>> for ServerView<'a> {
    fn player_cells(&self) -> Iter<'a, PlayerCell> {
        self.0.players.iter()
    }

    fn food_cells(&self) -> Iter<'a, FoodCell> {
        self.0.food.iter()
    }

    fn view_area(&self) -> Option<Circle> {
        self.0.player_view_radius()
    }
}
