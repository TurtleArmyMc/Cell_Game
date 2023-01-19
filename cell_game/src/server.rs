use std::{
    iter::{repeat, Filter, FlatMap, Map, Repeat, Zip},
    slice::Iter,
};

use rand::prelude::*;

use crate::{
    cells::{cell::Cell, food_cell::FoodCell, player_cell::PlayerCell},
    client_connection::{ClientConnection, PlayerInput},
    game_view::GameView,
    player::{Player, PlayerIdGenerator},
    player_connection::PlayerConnection,
    pos::{Circle, Point, Rect},
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

            let input = conn.connection().on_tick(ServerView {
                players: &self.players,
                food: &self.food,
                view_area,
            });

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

pub struct ServerView<'a> {
    players: &'a Vec<Player>,
    food: &'a Vec<FoodCell>,
    view_area: Option<Circle>,
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
                    std::slice::Iter<'a, Player>,
                    std::slice::Iter<'a, PlayerCell>,
                    fn(&'a Player) -> std::slice::Iter<'a, PlayerCell>,
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
