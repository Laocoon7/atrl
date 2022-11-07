use crate::game::prelude::*;

use iyes_loopless::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::SplashScreen)
            .init_resource::<DefaultAssets>()
            .add_plugin(CameraPlugin)
            .add_plugin(TilemapTestPlugin);
    }
}
