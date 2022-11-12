use crate::game::prelude::*;

pub fn load_first_map(
    mut commands: Commands,
    game_context: Res<GameContext>,
    mut map_loader: ResMut<MapLoader>,
    mut state: ResMut<CurrentGameState>,
    q_current_map: Query<Entity, With<CurrentMap>>,
) {
    map_loader.change_map(
        &mut commands,
        &game_context,
        WorldPosition { position: IVec3::ZERO },
        q_current_map,
    );

    state.set_next(&mut commands);
}
