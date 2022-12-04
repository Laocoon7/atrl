use crate::prelude::*;

#[derive(Resource)]
pub struct PlayerEntity {
    current_entity: Entity,
}

impl PlayerEntity {
    pub fn new(entity: Entity) -> Self {
        Self {
            current_entity: entity,
        }
    }

    pub fn current(&self) -> Entity { self.current_entity }
}
