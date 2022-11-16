use crate::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AIComponent {
    ai_type: AIType,
}

impl_new!(AIComponent, ai_type: AIType);

impl AIComponent {
    pub const fn human() -> Self { Self { ai_type: AIType::Player } }
    pub const fn scared() -> Self { Self { ai_type: AIType::Scared } }
    pub const fn aggressive() -> Self { Self { ai_type: AIType::Aggressive } }
}
