use crate::prelude::*;

#[derive(SystemLabel, Clone, Copy, PartialEq, Eq, Hash, Debug,)]
enum MapSystem {
    _Update,
    _Draw,
}

pub struct MapRendererPlugin {
    settings: MapRendererSettings,
}

impl MapRendererPlugin {
    pub fn new(chunk_size: impl Size2d,) -> Self {
        Self { settings: MapRendererSettings::new(chunk_size,), }
    }
}

impl Plugin for MapRendererPlugin {
    fn build(&self, app: &mut App,) {
        app
            // set chunk size for bevy_ecs_tilemap
            .insert_resource(TilemapRenderSettings { render_chunk_size: self.settings.chunk_size })
            // bevy_ecs_tilemap
            .add_plugin(TilemapPlugin);
    }
}
