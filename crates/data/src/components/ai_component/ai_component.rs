use crate::prelude::*;

#[derive(Component, Default)]
pub struct AIComponent {
    ai_type: AIType,
    pub preferred_action: Option<Box<dyn Action>>,
}

impl AIComponent {
    #[inline]
    pub const fn new(ai_type: AIType) -> Self {
        Self {
            ai_type,
            preferred_action: None,
        }
    }

    #[inline]
    pub fn get_action(&mut self) -> Option<BoxedAction> { std::mem::take(&mut self.preferred_action) }
}

impl AIComponent {
    pub const fn player() -> Self {
        Self {
            ai_type: AIType::Player,
            preferred_action: None,
        }
    }

    pub const fn scared() -> Self {
        Self {
            ai_type: AIType::Scared,
            preferred_action: None,
        }
    }

    pub const fn aggressive() -> Self {
        Self {
            ai_type: AIType::Aggressive,
            preferred_action: None,
        }
    }
}
