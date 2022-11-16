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
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: UVec2 { x: GRID_WIDTH, y: GRID_HEIGHT },
        })
        // bevy_ecs_tilemap
        .add_plugin(TilemapPlugin)
        // bevy_tileset
        .add_plugin(TilesetPlugin::default())
        // Holder for Handle<Tileset>'s so they don't get unloaded
        .init_resource::<LoadedTilesets>()
        // Loads the tilesets
        .add_enter_system(self.state_construct.clone(), load_tilesets)
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
    }
}
