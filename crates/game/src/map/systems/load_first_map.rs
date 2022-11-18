use crate::prelude::*;

pub(crate) fn load_first_map(
    mut commands: Commands,
    mut game_context: ResMut<GameContext>,
    tilesets: Tilesets,

    mut map_manager: ResMut<MapManager>,
) {
    if let Err(e) = map_manager.get_or_generate(
        &mut commands,
        &mut game_context,
        &tilesets,
        WorldPosition(IVec3::ZERO),
    ) {
        error!("{}", e);
    }
}
