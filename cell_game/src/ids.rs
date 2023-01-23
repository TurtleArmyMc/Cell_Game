use std::marker::PhantomData;

pub trait Id {
    fn new(id: u32) -> Self;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(u32);

impl Id for PlayerId {
    fn new(id: u32) -> Self {
        Self(id)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerCellId(u32);

impl Id for PlayerCellId {
    fn new(id: u32) -> Self {
        Self(id)
    }
}

pub(crate) struct IdGenerator<T: Id>(u32, PhantomData<T>);

impl<T: Id> IdGenerator<T> {
    pub(crate) fn new() -> Self {
        Self(0, PhantomData)
    }

    pub(crate) fn next(&mut self) -> T {
        self.0 += 1;
        T::new(self.0)
    }
}
