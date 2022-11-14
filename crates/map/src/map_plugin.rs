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
        app.add_plugin(TilesetPlugin::default())
            .init_resource::<LoadedTilesets>()
            .add_enter_system(self.state_construct.clone(), load_tilesets);
    }
}
