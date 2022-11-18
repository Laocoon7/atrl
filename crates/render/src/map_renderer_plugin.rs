use crate::prelude::*;

#[derive(SystemLabel, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum MapSystem {
    _Update,
    _Draw,
}

pub struct MapRendererPlugin<T> {
    state_construct: T,
    _state_running: T,
    settings: MapRendererSettings,
}

impl<T: StateNext> MapRendererPlugin<T> {
    pub fn new(state_construct: T, state_running: T, chunk_size: impl Size2d) -> Self {
        Self {
            state_construct,
            _state_running: state_running,
            settings: MapRendererSettings::new(chunk_size),
        }
    }

    pub fn add_tileset_file(mut self, file: &str) -> Self {
        self.settings.tileset_file_paths.push(file.to_string());
        self
    }

    pub fn add_tileset_folder(mut self, folder: &str) -> Self {
        self.settings.tileset_folder_paths.push(folder.to_string());
        self
    }
}

impl<T: StateNext> Plugin for MapRendererPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            // bevy_tileset
            .add_plugin(TilesetPlugin::default())
            // set chunk size for bevy_ecs_tilemap
            .insert_resource(TilemapRenderSettings { render_chunk_size: self.settings.chunk_size })
            // bevy_ecs_tilemap
            .add_plugin(TilemapPlugin)
            // hold `Handle<Tileset>`s so they don't get unloaded
            .insert_resource(LoadedTilesets::new(&self.settings))
            .add_enter_system(self.state_construct.clone(), load_tilesets);
    }
}
