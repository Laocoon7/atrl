use super::systems::*;
use crate::prelude::*;

pub struct CameraPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for CameraPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.state_running.clone(), spawn_main_camera);
    }
}
