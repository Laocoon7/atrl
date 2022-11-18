use crate::prelude::*;

#[derive(Bundle)]
pub struct ActorBundle {
    pub name: Name,
    pub health: Health,
    pub ai: AIComponent,
    pub position: WorldPosition,

    pub vision_component: Vision,
    pub movement_component: Movement,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
