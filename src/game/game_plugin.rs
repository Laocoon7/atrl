use crate::game::prelude::*;

use iyes_loopless::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::SplashScreen)
            .add_startup_system(spawn_main_camera)
            .init_resource::<DefaultAssets>();
    }
}
