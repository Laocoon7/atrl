use crate::prelude::*;

pub fn fov(
    q_map: Query<&Map>,
    manager: Res<MapManager>,
    q_player: Query<(&Transform, &FieldOfView, &Vision), With<Player>>,
    mut q_tile: Query<(&mut TileVisible, &TilePos)>,
    mut q_actors: Query<
        (Entity, &Transform, &mut Visibility),
        (With<AIComponent>, Without<Player>),
    >,
) {
    let mut visible_actors = Vec::new();
    for (player_pos, fov, vision_component) in q_player.iter() {
        if let Some(map) = q_map.iter().find(|m| m.world_position == manager.current_map) {
            let visibility_map =
                generate_visibility_map(&*map, player_pos.get(), fov.0, vision_component);

            // Tiles
            for (mut tile_vis, tile_pos) in q_tile.iter_mut() {
                if visibility_map.get_visible(tile_pos.as_ivec2())
                // | map.explored_tiles.contains(&tile_pos.as_uvec2())
                {
                    // TODO: Dim tiles in map.explored_tiles that aren't visible
                    // reveal tiles
                    tile_vis.0 = true;
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

    for entity in visible_actors.drain(..) {
        if let Ok((_, _, mut visible)) = q_actors.get_mut(entity) {
            visible.is_visible = true;
        }
    }
}

fn generate_visibility_map<P: fov::VisibilityProvider, R: Into<u32>>(
    visibility_provider: &P,
    origin: impl Point2d,
    range: R,
    vision_component: &Vision,
) -> VisibilityMap2d {
    let mut visibility_map = VisibilityMap2d::new_packer(visibility_provider.size());
    fov::compute(origin, range.into(), vision_component, visibility_provider, &mut visibility_map);
    //dump_visibility_map(&visibility_map);
    visibility_map
}

fn _dump_visibility_map(map: &VisibilityMap2d) {
    let size = map.size_packed();
    for y in 0..size.y {
        let mut s = String::new();
        for x in 0..size.x {
            let p = UVec2::new(x, y);
            if map.get_visible(p) {
                if map.get_opaque(p) {
                    s = format!("{}{}", s, "#");
                } else {
                    s = format!("{}{}", s, ".");
                }
            } else {
                s = format!("{}{}", s, "X");
            }
        }
        println!("{}", s);
    }
}
