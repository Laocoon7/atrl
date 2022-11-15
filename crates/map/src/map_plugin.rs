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
            .init_resource::<LoadedTilesets>()
            .add_enter_system(self.state_construct.clone(), load_tilesets);
        // .add_system_set(
        //     ConditionSet::new()
        //         .run_in_state(self.state_construct.clone())
        //         .with_system(build_map)
        //         .into(),
        // )
        // .add_enter_system(self.state_running.clone(), set_tile_custom_size)
        // .add_system_set_to_stage(
        //     CoreStage::Last,
        //     ConditionSet::new()
        //         .run_in_state(self.state_running.clone())
        //         .with_system(test_display)
        //         .into(),
        // );
    }
}
