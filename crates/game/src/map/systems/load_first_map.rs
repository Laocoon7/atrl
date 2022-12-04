use crate::prelude::*;
pub fn load_first_map(
    tilesets: Tilesets,
    mut commands: Commands,
    state: Res<CurrentGameState>,
    mut game_context: ResMut<GameContext>,
    mut map_manager: ResMut<MapManagerResource>,
) {
    info!("load_first_map");
    map_manager.get(
        &mut commands,
        &mut game_context,
        IVec3::ZERO,
        &tilesets,
        true,
    );
    state.set_next(&mut commands)
}
