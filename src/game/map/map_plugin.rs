use crate::app_settings::*;
use crate::game::prelude::*;

pub struct MapPlugin<T> {
    pub state_running: T,
}
impl<T: StateNext> Plugin for MapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapTileTemplates>()
            .insert_resource(CurrentMap(gen_map()))
            .add_enter_system(self.state_running.clone(), create_tilemaps)
            .add_system(update_map_tiles.run_in_state(self.state_running.clone()));
    }
}

fn gen_map() -> Map { Map::new([GRID_WIDTH, GRID_HEIGHT]) }
