use atrl_data::fov::VisibilityMapUtility;

use crate::prelude::*;

pub fn fov(
    mut map: Query<&mut Map>,
    manager: Res<MapManager>,
    player_q: Query<(&Transform, &FieldOfView, &Vision), With<Player>>,
    mut tile_q: Query<(&mut TileVisible, &TilePos)>,
    mut vis_q: Query<(&Transform, &mut Visibility), (With<AIComponent>, Without<Player>)>,
) {
    if let Ok((player_pos, fov, vision_component)) = player_q.get_single() {
        if let Some(mut map) = map.iter_mut().find(|m| m.world_position == manager.current_map) {
            map.visibility_map.clear_visible();
            map.visibility_map.clear_opaque();

            fov::compute(player_pos.get(), fov.0, vision_component, &mut *map);

            // Tiles
            for (mut tile_vis, tile_pos) in tile_q.iter_mut() {
                if map.visibility_map.visible_at(tile_pos.as_ivec2()) {
                    // reveal tiles
                    tile_vis.0 = true;
                } else {
                    // hide tiles
                    tile_vis.0 = false;
                }
            }

            // Actors
            for (e_pos, mut e_vis) in vis_q.iter_mut() {
                if map.visibility_map.visible_at(e_pos.get()) {
                    e_vis.is_visible = true;
                } else {
                    e_vis.is_visible = false;
                }
            }
        }
    }
}
