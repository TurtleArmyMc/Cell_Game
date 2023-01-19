use crate::{
    cells::{cell::Cell, player_cell::PlayerCell},
    pos::Point,
};

pub type PlayerId = u32;

pub struct Player {
    id: PlayerId,
    cells: Vec<PlayerCell>,
}

impl Player {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            cells: Vec::new(),
        }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn total_mass(&self) -> f64 {
        self.cells.iter().map(|c| c.mass()).sum()
    }

    pub fn set_move_to(&mut self, pos: Point) {
        for cell in self.cells.iter_mut() {
            cell.move_towards_point(pos)
        }
    }

    pub fn cells(&self) -> &Vec<PlayerCell> {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut Vec<PlayerCell> {
        &mut self.cells
    }
}

pub(crate) struct PlayerIdGenerator(PlayerId);

impl PlayerIdGenerator {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn next(&mut self) -> PlayerId {
        self.0 += 1;
        self.0
    }
}
