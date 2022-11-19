use crate::prelude::*;

static PLAYER_SPAWN: &str = "player_spawn";

pub struct SpawnerPlugin<T> {
    pub state_construct: T,
}

impl<T: StateNext> Plugin for SpawnerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_exit_system(self.state_construct.clone(), spawn_player.label(PLAYER_SPAWN))
            .add_exit_system(self.state_construct.clone(), spawn_ai.after(PLAYER_SPAWN));
    }
}
