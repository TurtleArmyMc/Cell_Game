pub type PlayerId = u32;

pub struct PlayerInfo {
    id: PlayerId,
    name: String,
}

impl PlayerInfo {
    pub(crate) fn new(name: String, id_gen: &mut PlayerIdGenerator) -> Self {
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

pub(crate) struct PlayerIdGenerator(PlayerId);

impl PlayerIdGenerator {
    pub(crate) fn new() -> Self {
        Self(0)
    }

    fn next(&mut self) -> PlayerId {
        self.0 += 1;
        self.0
    }
}
