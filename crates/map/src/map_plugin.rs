use crate::prelude::*;

#[derive(SystemLabel, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum MapSystem {
    _Update,
    _Draw,
}

pub struct MapPlugin<T> {
    pub state_construct: T,
    pub state_running: T,
}

impl<T: StateNext> Plugin for MapPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            // bevy_ecs_tilemap
            .add_plugin(TilemapPlugin)
            // bevy_tileset
            .add_plugin(TilesetPlugin::default())
            // Holder for Handle<Tileset>'s so they don't get unloaded
            .init_resource::<LoadedTilesets>()
            // Loads the tilesets
            .add_enter_system(self.state_construct.clone(), load_tilesets)
            //
            // call create_tilemap and set next stage (this must run multiple times to ensure
            // tilesets are loaded)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(self.state_construct.clone())
                    .with_system(build_map)
                    .into(),
            );
    }
}
