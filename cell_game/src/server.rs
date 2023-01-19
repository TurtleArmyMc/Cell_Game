use rand::prelude::*;

use crate::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    client_connection::{ClientConnection, PlayerInput},
    player::{Player, PlayerIdGenerator},
    player_connection::PlayerConnection,
    pos::{Circle, Point, Rect},
    server_view::ServerView,
};

pub struct GameServer {
    players: Vec<Player>,
    food: Vec<FoodCell>,
    bounds: Rect,

    player_id_gen: PlayerIdGenerator,

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
            connections: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        for cell in self
            .players
            .iter_mut()
            .flat_map(|p| p.cells_mut().iter_mut())
        {
            cell.move_player(self.bounds)
        }

        for conn in self.connections.iter_mut() {
            let player = self
                .players
                .iter_mut()
                .filter(|p| p.id() == conn.id())
                .next()
                .expect("could not find player for connection");
            let view_area = Self::player_view_radius(player.cells());

            let input =
                conn.connection()
                    .on_tick(ServerView::new(&self.players, &self.food, view_area));

            let player = self
                .players
                .iter_mut()
                .filter(|p| p.id() == conn.id())
                .next()
                .expect("could not find player for connection");

            if let Some(PlayerInput { move_to }) = input {
                Self::set_move_to(player.cells_mut(), move_to)
            }
        }
    }

    pub fn add_connection(
        &mut self,
        conn: Box<dyn for<'a> ClientConnection<'a, V = ServerView<'a>>>,
    ) {
        let player_id = self.player_id_gen.next();

        self.connections
            .push(PlayerConnection::new(conn, player_id));

        let mut player = Player::new(player_id);
        player
            .cells_mut()
            .push(PlayerCell::spawn_new(self.bounds.center()));

        self.players.push(player);
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
