use crate::prelude::*;

#[derive(Component)]
pub struct LastUpdate {
    pub value: f64,
}

pub(crate) fn build_map(
    mut commands: Commands,
    state: Res<CurrentGameState>,
    tilesets: Tilesets,
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    if let Some(tileset) = tilesets.get_by_id(&0) {
        create_tilemap(&mut commands, [GRID_WIDTH, GRID_HEIGHT], tileset, &array_texture_loader);

        state.set_next(&mut commands);
    }
}