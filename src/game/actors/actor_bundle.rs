use crate::game::prelude::*;

#[derive(Bundle)]
pub struct ActorBundle {
    name: Name,
    position: LocalPosition,
    health: Health,
    ai: AIComponent,
}
