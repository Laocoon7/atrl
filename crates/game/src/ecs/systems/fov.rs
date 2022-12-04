use crate::prelude::*;
pub fn fov(
    mut map_manager: MapManager,
    mut q_tile: Query<(&mut TileVisible, &mut TileColor, &TilePos)>,
    player_entity: Res<PlayerEntity>,
    mut q_vision: Query<(&Position, &FieldOfView, &Vision, &mut Visibility)>,
    q_blocks_vision: Query<&BlocksVision>,
) {
    let Ok((player_position, fov, vision_component, _)) = q_vision.get(player_entity.current()) else {
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

    for position in visibility_map.get_all().iter() {
        map_manager.mark_explored(*position);
    }

    for (position, _fov, _vision, mut visibility) in q_vision.iter_mut() {
        if visibility_map.get_visible(*position) {
            visibility.is_visible = true;
        } else {
            visibility.is_visible = false;
        }
    }

    //// Tiles
    //for (mut tile_vis, mut tile_col, tile_pos) in q_tile.iter_mut() {
    //    if visibility_map.get_visible(tile_pos) |
    //        map.explored_tiles.contains(&tile_pos.as_uvec2())
    //    {
    //        tile_vis.0 = true;
    //        tile_col.0 = *tile_col.0.set_a(1.0);
    //    } else {
    //        tile_col.0 = *tile_col.0.set_a(0.15);
    //    }
    //}

}
