use rand::prelude::*;

use crate::{
    color::HSL,
    ids::{IdGenerator, PlayerId},
};

#[derive(Clone)]
pub struct PlayerInfo {
    id: PlayerId,
    name: String,
    color: HSL,
}

impl PlayerInfo {
    pub(crate) fn new(name: String, id_gen: &mut IdGenerator<PlayerId>) -> Self {
        Self {
            id: id_gen.next(),
            name,
            color: HSL::new(random(), 255, 130),
        }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> HSL {
        self.color
    }
}
