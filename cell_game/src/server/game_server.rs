use std::iter::repeat_with;

use crate::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    client_connection::{ClientConnection, PlayerInput},
    player_info::{PlayerId, PlayerIdGenerator, PlayerInfo},
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
    pub const TICK_RATE: usize = 60;
    const GAME_BOUNDS: Rect = Rect {
        top_left: Point { x: 0.0, y: 0.0 },
        width: 1920.0,
        height: 1080.0,
    };
    const VIEW_RADIUS_MULTIPLIER: f64 = 30.0;
    const FOOD_AMOUNT: usize = 100;

    pub fn new() -> Self {
        let bounds = Self::GAME_BOUNDS;
        let food = Self::n_food(bounds, Self::FOOD_AMOUNT).collect();
        Self {
            players: Vec::new(),
            food,
            bounds,
            player_id_gen: PlayerIdGenerator::new(),
            player_infos: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        self.move_players();
        self.feed_food();
        self.remove_mass();
        self.handle_connections();
    }

    pub fn connect_player(
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

    fn move_players(&mut self) {
        for cell in self.players.iter_mut() {
            cell.move_player(self.bounds)
        }
    }

    fn feed_food(&mut self) {
        let mut eaten = 0;
        for player_cell in self.players.iter_mut() {
            let hitbox = player_cell.hitbox();
            self.food.retain(|food_cell| {
                if hitbox.contains_point(food_cell.pos()) {
                    player_cell.add_mass(food_cell.mass());
                    eaten += 1;
                    false
                } else {
                    true
                }
            })
        }
        self.food.extend(Self::n_food(self.bounds, eaten))
    }

    fn remove_mass(&mut self) {
        for player_cell in self.players.iter_mut() {
            player_cell.lose_mass();
        }
    }

    fn handle_connections(&mut self) {
        for conn in self.connections.iter_mut() {
            let owner = conn.id();
            let view_area = Self::player_view_area(&self.players, owner);
            let input = conn.connection().on_tick(ServerView::new(
                &self.players,
                &self.food,
                &self.player_infos,
                view_area,
                owner,
            ));

            if let Some(PlayerInput { move_to }) = input {
                Self::set_move_to(
                    self.players.iter_mut().filter(|cell| cell.owner() == owner),
                    move_to,
                )
            }
        }
    }

    fn n_food(bounds: Rect, n: usize) -> impl Iterator<Item = FoodCell> {
        repeat_with(move || FoodCell::new_within(bounds)).take(n)
    }

    fn set_move_to<'a, I: Iterator<Item = &'a mut PlayerCell>>(players: I, dest: Point) {
        for p in players {
            p.move_towards_point(dest)
        }
    }

    fn player_view_area(players: &Vec<PlayerCell>, owner: PlayerId) -> Option<Circle> {
        players
            .iter()
            .find(|cell| cell.owner() == owner)
            .map(|p| p.hitbox().scale_centered(Self::VIEW_RADIUS_MULTIPLIER))
    }
}
