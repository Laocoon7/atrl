use crate::prelude::*;

pub fn fov<'w, 's>(
    mut map_manager: MapManager,
    player_entity: Res<PlayerEntity>,
    q_vision: Query<(&Position, &FieldOfView, &Vision)>,
    q_blocks_vision: Query<'w, 's, &'static BlocksVision>,
) {
    let Ok((player_position, fov, vision_component)) = q_vision.get(player_entity.current()) else {
        error!("No player");
        return;
    };

    let mut visibility_map = VisibilityMap::new();
    Fov::Shadowcast.compute(
        *player_position,
        vision_component.0,
        fov.0,
        &mut map_manager,
        &q_blocks_vision,
        &mut visibility_map,
    );

    map_manager.set_visibility(visibility_map);
}
