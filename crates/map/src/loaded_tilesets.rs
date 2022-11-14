use crate::prelude::*;

#[derive(Resource, Default)]
pub(crate) struct LoadedTilesets {
    pub default_handle: Option<Handle<Tileset>>,
    pub other_handles: Vec<Handle<Tileset>>,
}
