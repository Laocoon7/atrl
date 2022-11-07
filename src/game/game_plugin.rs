use crate::game::prelude::*;

use iyes_loopless::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::default())
            .init_resource::<DefaultAssets>()
            .add_plugin(CameraPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(TilemapTestPlugin);
    }
}
