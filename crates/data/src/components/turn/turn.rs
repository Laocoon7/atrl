use crate::prelude::*;

#[derive(Component)]
pub struct MyTurn;

#[derive(Component)]
pub struct WaitingForTurn {
    pub next_turn_tick: u64,
}

impl Default for WaitingForTurn {
    fn default() -> Self { Self { next_turn_tick: 1 } }
}
