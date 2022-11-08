use crate::game::prelude::*;

pub struct GameContext {
    pub random: Random,
}

impl Default for GameContext {
    fn default() -> Self {
        GameContext { random: Random::from_entropy() }
    }
}