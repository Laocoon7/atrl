use crate::prelude::*;

pub fn load_first_map(
    mut commands: Commands,
    game_context: Res<GameContext>,
    q_current_map: Query<Entity, With<CurrentMap>>,
) {
    let mut loaded_maps = MapLoader { maps: HashMap::new() };

    loaded_maps.change_map(
        &mut commands,
        &game_context,
        WorldPosition { position: IVec3::ZERO },
        q_current_map,
    );

    commands.insert_resource(loaded_maps);
}
