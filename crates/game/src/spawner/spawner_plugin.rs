use crate::prelude::*;

static PLAYER_SPAWN: &str = "player_spawn";

pub struct SpawnerPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for SpawnerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.state_running.clone(), spawn_player.label(PLAYER_SPAWN))
            .add_enter_system(self.state_running.clone(), spawn_ai.after(PLAYER_SPAWN));
    }
}
