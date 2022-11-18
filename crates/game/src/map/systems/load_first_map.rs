use crate::prelude::*;

pub(crate) fn load_first_map(
    mut commands: Commands,
    mut game_context: ResMut<GameContext>,
    tilesets: Tilesets,

    mut map_manager: ResMut<MapManager>,
) {
    let tileset_id = match Prng::from_entropy().coin() {
        false => 0,
        true => 1,
    };

    if let Err(e) = map_manager.get_or_generate(
        &mut commands,
        &mut game_context,
        None,
        Some(tileset_id),
        &tilesets,
        WorldPosition(IVec3::ZERO),
    ) {
        error!("{}", e);
    }
}
