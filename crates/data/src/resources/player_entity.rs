use crate::prelude::*;

#[derive(Resource)]
pub struct PlayerEntity {
    current_entity: Entity,
}

impl PlayerEntity {
    #[inline(always)]
    pub const fn new(entity: Entity) -> Self {
        Self {
            current_entity: entity,
        }
    }

    pub const fn current(&self) -> Entity { self.current_entity }
}
