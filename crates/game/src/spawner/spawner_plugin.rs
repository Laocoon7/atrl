use crate::prelude::*;

static PLAYER_SPAWN: &str = "player_spawn";

pub struct SpawnerPlugin<T> {
    pub state_construct_setup: T,
}

impl<T: StateNext> Plugin for SpawnerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.state_construct_setup, spawn_player.label(PLAYER_SPAWN))
            .add_enter_system(self.state_construct_setup, spawn_ai.after(PLAYER_SPAWN));
    }
}
