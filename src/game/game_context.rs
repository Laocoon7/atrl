use crate::game::prelude::internal::*;
use crate::prelude::*;

pub struct GameContext {
    pub random: Lazy<Mutex<Random>>,
    pub world_bounds: Rectangle,
}

impl Default for GameContext {
    fn default() -> Self {
        Self {
            random: Lazy::new(|| Mutex::new(Random::from_entropy())),
            world_bounds: Rectangle { min: (-5, -5).into(), max: (5, 5).into() },
        }
    }
}

impl GameContext {
    pub fn get_rng(&self) -> MutexGuard<Random> { self.random.lock() }

    pub fn is_valid_world_position(&self, world_position: WorldPosition) -> bool {
        self.world_bounds.contains(world_position.xy())
    }
}
