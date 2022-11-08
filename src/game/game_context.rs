use crate::game::prelude::*;
use parking_lot::{Mutex, MutexGuard};

pub struct GameContext {
    pub random: Mutex<Random>,
}

impl Default for GameContext {
    fn default() -> Self { Self { random: Mutex::new(Random::from_entropy()) } }
}

impl GameContext {
    pub fn get_rng(&self) -> MutexGuard<Random> { self.random.lock() }
}
