use crate::prelude::*;

// Stupid simple workaround for the source project duck typing.

#[derive(Clone, Copy)]
pub enum Target {
    None,
    Player(Handle<Player>),
    Goal(Handle<Goal>),
}

impl Target {
    pub fn load<'a>(&self, pools: &'a Pools) -> &'a dyn Targetable {
        match self {
            Self::Player(handle) => pools.players.borrow(*handle),
            Self::Goal(handle) => pools.goals.borrow(*handle),
            Self::None => panic!(),
        }
    }
}
