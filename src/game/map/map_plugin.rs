use crate::game::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) { app.init_resource::<MapTileTemplates>(); }
}
