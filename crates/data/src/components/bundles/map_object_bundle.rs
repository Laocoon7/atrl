use crate::prelude::*;

#[derive(Bundle)]
pub struct MapObjectBundle {
    pub name: Name,
    pub position: Position,

    pub blocks_movement: BlocksMovement,
    pub blocks_vision: BlocksVision,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
