use crate::game::prelude::*;

#[derive(Bundle)]
pub struct ActorBundle {
    pub name: Name,
    pub position: LocalPosition,
    pub health: Health,
    pub ai: AIComponent,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl ActorBundle {
    #[allow(dead_code)]
    pub fn new<Str: ToString>(
        name: Str,
        sprite: SpriteSheetBundle,
        position: LocalPosition,
        health: Health,
        ai: AIComponent,
    ) -> Self {
        Self { position, sprite, health, ai, name: Name::new(name.to_string()) }
    }
}
