use crate::prelude::*;

#[derive(Resource)]
pub struct GameContext {
    pub random: Random,
    pub world_bounds: Rectangle,
    pub map_manager_random: Random,
}

impl Default for GameContext {
    fn default() -> Self {
        let mut random = Random::from_entropy();
        let map_manager_random = Random::new(random.prng.next_u64());

        Self { map_manager_random, random, world_bounds: Rectangle::new((-5, -5), (5, 5)) }
    }
}

impl GameContext {
    pub fn is_valid_world_position(&self, world_position: WorldPosition) -> bool {
        self.world_bounds.contains(world_position.xy())
    }
}
