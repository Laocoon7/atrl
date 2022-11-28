use std::collections::VecDeque;

use crate::prelude::*;

#[derive(Default, Resource)]
pub struct ActionQueue {
    actions: VecDeque<ActionType>,
}

impl ActionQueue {
    pub fn add_action(&mut self, action: ActionType) { self.actions.push_back(action); }

    pub fn get_action(&mut self) -> Option<ActionType> { self.actions.pop_front() }
}
