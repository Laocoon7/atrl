use super::systems::*;
use crate::game::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) { app.add_startup_system(spawn_main_camera); }
}
