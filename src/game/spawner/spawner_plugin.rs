use super::spawn_player;
use atrl_engine::{AppLooplessStateExt, Plugin, StateNext};

pub struct SpawnerPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for SpawnerPlugin<T> {
    fn build(&self, app: &mut atrl_engine::App) {
        app.add_enter_system(self.state_running.clone(), spawn_player);
    }
}
