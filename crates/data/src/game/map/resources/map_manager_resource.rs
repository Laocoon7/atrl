use crate::prelude::*;
#[derive(Resource)]
pub struct MapManagerResource {
    pub current_map: (WorldPosition, Map),
    pub loaded_maps: HashMap<WorldPosition, Map>,
}

// Constructor
impl MapManagerResource {
    pub fn new(world_position: WorldPosition, map: Map) -> Self {
        Self {
            current_map: (world_position, map),
            loaded_maps: HashMap::new(),
        }
    }
}
