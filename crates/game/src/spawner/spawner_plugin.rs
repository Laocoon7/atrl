use crate::prelude::*;

static PLAYER_SPAWN: &str = "player_spawn";

pub struct SpawnerPlugin<T> {
    pub state_construct: T,
}

impl<T: StateNext> Plugin for SpawnerPlugin<T> {
    fn build(&self, app: &mut App) {
        // TODO: See note in game_plugin where this gets added.
        app.add_enter_system(self.state_construct, spawn_player.label(PLAYER_SPAWN))
            .add_enter_system(self.state_construct, spawn_ai.after(PLAYER_SPAWN));
        //app.add_exit_system(self.state_construct, spawn_player.label(PLAYER_SPAWN))
        //    .add_exit_system(self.state_construct, spawn_ai.after(PLAYER_SPAWN));
    }
}
