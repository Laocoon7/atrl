use crate::prelude::*;

pub trait StateNext: StateData {
    fn next(&self) -> Option<Self>;
}

pub trait StateSetNext {
    fn set_next(&self, commands: &mut Commands);
}

impl<T: StateNext> StateSetNext for Res<'_, CurrentState<T>> {
    fn set_next(&self, commands: &mut Commands) {
        let current = &self.0;

        current.next().map_or_else(
            || {
                bevy::log::error!("no next state for {:?}.", current);
            },
            |next| {
                bevy::log::info!("transitioning state from {:?} to {:?}", current, next);
                switch_in_game_state!(commands, next);
            },
        )
    }
}

impl<T: StateNext> StateSetNext for ResMut<'_, CurrentState<T>> {
    fn set_next(&self, commands: &mut Commands) {
        let current = &self.0;

        current.next().map_or_else(
            || {
                bevy::log::error!("no next state for {:?}.", current);
            },
            |next| {
                bevy::log::info!("transitioning state from {:?} to {:?}", current, next);
                switch_in_game_state!(commands, next);
            },
        )
    }
}
