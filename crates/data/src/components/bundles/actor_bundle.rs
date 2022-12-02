use crate::prelude::*;
#[derive(Bundle)]
pub struct ActorBundle {
    pub name: Name,
    pub mob: Mob,
    pub health: Health,
    pub ai: AIComponent,
    pub position: Position,

    pub fov: FieldOfView,
    pub vision_component: Vision,
    pub movement_component: Movement,

    pub target_visualizer: TargetVisualizer,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
