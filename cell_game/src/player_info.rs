use crate::ids::{IdGenerator, PlayerId};

#[derive(Clone)]
pub struct PlayerInfo {
    id: PlayerId,
    name: String,
}

impl PlayerInfo {
    pub(crate) fn new(name: String, id_gen: &mut IdGenerator<PlayerId>) -> Self {
        Self {
            id: id_gen.next(),
            name,
        }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
