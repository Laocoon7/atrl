use crate::prelude::*;

#[derive(Component, Default)]
pub struct AIComponent {
    ai_type: AIType,
    pub preferred_action: Option<BoxedAction>,
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
    pub fn get_action(&self) -> Option<Box<dyn AtrlAction>> {
        let action = self.preferred_action.as_deref()?;
        Some(dyn_clone::clone_box(action))
    }

    #[inline]
    pub fn take_action(&mut self) -> Option<BoxedAction> { self.preferred_action.take() }

    #[inline]
    pub fn set_action(&mut self, action: BoxedAction) { self.preferred_action = Some(action); }

    #[inline]
    pub fn clear_action(&mut self) { self.preferred_action = None; }

    #[inline]
    pub fn has_action(&mut self) -> bool { self.preferred_action.is_some() }
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
