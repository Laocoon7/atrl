use crate::prelude::*;

pub fn load_first_map(
    mut commands: Commands,
    mut game_context: ResMut<GameContext>,
    tilesets: Tilesets,

    mut map_manager: ResMut<MapManager>,
) {
    let tileset_id = Prng::from_entropy().range(0..TILESET_TERRAIN_IDS_TABLE.len() as u32) as usize;
    let tileset_id = TILESET_TERRAIN_IDS_TABLE[tileset_id];

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
