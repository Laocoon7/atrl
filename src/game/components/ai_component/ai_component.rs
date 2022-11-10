use crate::game::prelude::*;

#[derive(Component)]
pub struct AIComponent {
    ai_type: AIType,
}

impl_new!(AIComponent, ai_type: AIType);

impl AIComponent {
    pub fn human() -> Self { Self { ai_type: AIType::Human } }
    pub fn scared() -> Self { Self { ai_type: AIType::Scared } }
    pub fn aggressive() -> Self { Self { ai_type: AIType::Aggressive } }
}
