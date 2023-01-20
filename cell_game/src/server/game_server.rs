use rand::prelude::*;

use crate::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    client_connection::{ClientConnection, PlayerInput},
    player_info::{PlayerIdGenerator, PlayerInfo},
    pos::{Circle, Point, Rect},
};

use super::{player_connection::PlayerConnection, server_view::ServerView};

pub struct GameServer {
    players: Vec<PlayerCell>,
    food: Vec<FoodCell>,
    bounds: Rect,

    player_id_gen: PlayerIdGenerator,
    player_infos: Vec<PlayerInfo>,

    connections: Vec<PlayerConnection>,
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
            player_id_gen: PlayerIdGenerator::new(),
            player_infos: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        for cell in self.players.iter_mut() {
            cell.move_player(self.bounds)
        }

        for conn in self.connections.iter_mut() {
            let input = conn.connection().on_tick(ServerView::new(
                &self.players,
                &self.food,
                &self.player_infos,
                Self::player_view_radius(&self.players),
            ));

            if let Some(PlayerInput { move_to }) = input {
                Self::set_move_to(
                    self.players
                        .iter_mut()
                        .filter(|cell| cell.owner() == conn.id()),
                    move_to,
                )
            }
        }
    }

    pub fn add_connection(
        &mut self,
        name: String,
        conn: Box<dyn for<'a> ClientConnection<'a, V = ServerView<'a>>>,
    ) {
        let player_info = PlayerInfo::new(name, &mut self.player_id_gen);

        self.connections
            .push(PlayerConnection::new(conn, player_info.id()));

        self.players.push(PlayerCell::spawn_new(
            self.bounds.center(),
            player_info.id(),
        ));

        self.player_infos.push(player_info);
    }

    pub fn spawn_food(&mut self) {
        self.food.push(FoodCell::spawn_new(Point {
            x: self.bounds.min_x() + (self.bounds.width * random::<f64>()),
            y: self.bounds.min_y() + (self.bounds.height * random::<f64>()),
        }))
    }

    fn set_move_to<'a, I: Iterator<Item = &'a mut PlayerCell>>(players: I, dest: Point) {
        for p in players {
            p.move_towards_point(dest)
        }
    }

    fn player_view_radius(players: &Vec<PlayerCell>) -> Option<Circle> {
        players
            .first()
            .map(|p| p.hitbox().scale_centered(Self::VIEW_RADIUS_MULTIPLIER))
    }
}
