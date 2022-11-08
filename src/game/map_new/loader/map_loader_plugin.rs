use crate::app_settings::*;
use crate::game::prelude::*;

pub struct MapLoaderPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for MapLoaderPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentMap(Map::new([GRID_WIDTH, GRID_HEIGHT])));
    }
}
