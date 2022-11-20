use crate::prelude::*;

pub fn fov(
    q_map: Query<&Map>,
    manager: Res<MapManager>,
    q_player: Query<(&Transform, &FieldOfView, &Vision), With<Player>>,
    mut q_tile: Query<(&mut TileVisible, &mut TileColor, &TilePos)>,
    mut q_actors: Query<
        (Entity, &Transform, &mut Visibility),
        (With<AIComponent>, Without<Player>),
    >,
) {
    let mut visible_actors = Vec::new();
    for (player_pos, fov, vision_component) in q_player.iter() {
        if let Some(map) = q_map.iter().find(|m| m.world_position == manager.current_map) {
            let visibility_map =
                generate_visibility_map(map, player_pos.get(), fov.0, vision_component);

            // Tiles
            for (mut tile_vis, mut tile_col, tile_pos) in q_tile.iter_mut() {
                if visibility_map.get_visible(tile_pos.as_ivec2())
                    | map.explored_tiles.contains(&tile_pos.as_uvec2())
                {
                    tile_vis.0 = true;
                    tile_col.0 = *tile_col.0.set_a(1.0);
                } else {
                    tile_col.0 = *tile_col.0.set_a(0.15);
                }
            }

            // Actors
            for (entity, e_pos, mut e_vis) in q_actors.iter_mut() {
                if visibility_map.get_visible(e_pos.get()) {
                    visible_actors.push(entity);
                } else {
                    e_vis.is_visible = false;
                }
            }
        }
    }

    for entity in visible_actors.into_iter() {
        if let Ok((_, _, mut visible)) = q_actors.get_mut(entity) {
            visible.is_visible = true;
        }
    }
}

fn generate_visibility_map<P: VisibilityProvider, R: Into<u32>>(
    visibility_provider: &P,
    origin: impl Point2d,
    range: R,
    vision_component: &Vision,
) -> VisibilityMap2d {
    let mut visibility_map = VisibilityMap2d::new_packer(visibility_provider.size());
    fov::compute(origin, range.into(), vision_component, visibility_provider, &mut visibility_map);
    visibility_map
}
