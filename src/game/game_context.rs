use crate::game::prelude::*;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};

pub struct GameContext {
    pub random: Lazy<Mutex<Random>>,
}

impl Default for GameContext {
    fn default() -> Self { Self { random: Lazy::new(|| Mutex::new(Random::from_entropy())) } }
}

impl GameContext {
    pub fn get_rng(&self) -> MutexGuard<Random> { self.random.lock() }
}
