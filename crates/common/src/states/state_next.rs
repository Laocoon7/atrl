use crate::prelude::*;

pub trait StateNext: StateData + Copy + Default {
    fn next(&self,) -> Option<Self,>;
}

pub trait StateSetNext {
    fn set_next(&self, commands: &mut Commands,);
    fn go_to(&self, commands: &mut Commands, state: impl StateNext,);
}

impl<T: StateNext,> StateSetNext for Res<'_, CurrentState<T,>,> {
    fn set_next(&self, commands: &mut Commands,) {
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

    fn go_to(&self, commands: &mut Commands, state: impl StateNext,) {
        let current = &self.0;
        bevy::log::info!("transitioning state from {:?} to {:?}", current, state);
        switch_in_game_state!(commands, state);
    }
}

impl<T: StateNext,> StateSetNext for ResMut<'_, CurrentState<T,>,> {
    fn set_next(&self, commands: &mut Commands,) {
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

    fn go_to(&self, commands: &mut Commands, state: impl StateNext,) {
        let current = &self.0;
        bevy::log::info!("transitioning state from {:?} to {:?}", current, state);
        switch_in_game_state!(commands, state);
    }
}
