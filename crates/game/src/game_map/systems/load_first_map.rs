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
        WorldPosition { position: IVec3 { x: 0, y: 0, z: 0 } },
    ) {
        error!("{}", e);
    }
}
