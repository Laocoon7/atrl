use crate::prelude::*;
pub fn fov(
    manager: Res<MapManager>,
    mut q_tile: Query<(&mut TileVisible, &mut TileColor, &TilePos)>,
    q_player: Query<(&Transform, &FieldOfView, &Vision), With<Player>>,
    mut q_actors: Query<(&Transform, &mut Visibility), (With<AIComponent>, Without<Player>)>,
) {
    for (player_pos, fov, vision_component) in q_player.iter() {
        if let Some(map) = manager.get_current_map() {
            let mut visibility_map = VisibilityMap::new(map.size);
            Fov::Shadowcast.compute(
                player_pos.get(),
                vision_component.0,
                fov.0,
                map,
                &mut visibility_map,
            );

            // Tiles
            for (mut tile_vis, mut tile_col, tile_pos) in q_tile.iter_mut() {
                if visibility_map.get_visible(tile_pos.as_ivec2()) |
                    map.explored_tiles.contains(&tile_pos.as_uvec2())
                {
                    tile_vis.0 = true;
                    tile_col.0 = *tile_col.0.set_a(1.0);
                } else {
                    tile_col.0 = *tile_col.0.set_a(0.15);
                }
            }

            // Actors
            for (e_pos, mut e_vis) in q_actors.iter_mut() {
                if visibility_map.get_visible(e_pos.get()) {
                    e_vis.is_visible = true;
                } else {
                    e_vis.is_visible = false;
                }
            }
        }
    }
}