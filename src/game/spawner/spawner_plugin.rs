use crate::game::prelude::internal::*;
use crate::prelude::*;

pub struct SpawnerPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for SpawnerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.state_running.clone(), spawn_player);
    }
}
