use crate::prelude::*;

#[derive(SystemLabel, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum MapSystem {
    _Update,
    _Draw,
}

pub struct MapPlugin<T> {
    pub state_construct: T,
    pub state_running: T,
    tilemap_settings: TilemapSettings,
}

impl<T: StateNext> MapPlugin<T> {
    pub fn new(state_construct: T, state_running: T, tilemap_settings: TilemapSettings) -> Self {
        Self { state_construct, state_running, tilemap_settings }
    }
}

impl<T: StateNext> Plugin for MapPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            // bevy_tileset
            .add_plugin(TilesetPlugin::default())
            // set chunk size for bevy_ecs_tilemap
            .insert_resource(TilemapRenderSettings {
                render_chunk_size: self.tilemap_settings.chunk_size,
            })
            // bevy_ecs_tilemap
            .add_plugin(TilemapPlugin)
            // hold `Handle<Tileset>`'s so they don't get unloaded
            .insert_resource(LoadedTilesets::new(&self.tilemap_settings.tileset_folders))
            // Loads the tilesets
            .add_enter_system(self.state_construct.clone(), load_tilesets);

        // Move these to Game::Map
        /*
                    .add_enter_system(self.state_construct.clone(), load_first_map)
                    .add_system_set(
                        ConditionSet::new()
                            .run_in_state(self.state_running.clone())
                            .with_system(gen_tilemaps_for_maps)
                            .into(),
                    )
                    .add_system_set_to_stage(
                        CoreStage::Last,
                        ConditionSet::new()
                            .run_in_state(self.state_running.clone())
                            .with_system(update_tilemaps)
                            .into(),
                    );
        */
    }
}
