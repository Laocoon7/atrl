use crate::prelude::*;

pub fn update_animations(
    time: Res<Time>,
    mut q_map_renderer: Query<&mut MapRenderer>,
    mut q_animated_tiles: Query<&mut AnimatedTile, Without<ForegroundTile>>,
) {
    let elapsed_time_64 = time.elapsed_seconds_f64();

    for mut map_renderer in q_map_renderer.iter_mut() {
        let mut entities = Vec::new();
        for (layer, grid) in &map_renderer.entities {
            // skip background layers
            if layer % 2 == 0 {
                continue;
            }

            for y in 0..grid.height() {
                for x in 0..grid.width() {
                    let foreground_entity = map_renderer.get_entity(*layer, (x, y));
                    let background_entity = map_renderer.get_entity(layer - 1, (x, y));
                    if foreground_entity.is_some() && background_entity.is_some() {
                        entities.push((foreground_entity.unwrap(), background_entity.unwrap()));
                    }
                }
            }
        }

        let mut map_context = map_renderer.get_context();

        for (foreground_entity, background_entity) in entities {
            if let Ok(mut tile) = q_animated_tiles.get_mut(foreground_entity) {
                if calc_current_frame(&mut tile, elapsed_time_64) {
                    let frame = &tile.foreground_tiles[tile.current_index];
                    let texture_atlas_handle = &frame.texture_atlas;
                    let index = frame.index;
                    let foreground_color = &frame.color;
                    let background_color = &frame.bg_color;

                    map_context.set_raw(
                        &foreground_entity,
                        texture_atlas_handle,
                        index,
                        foreground_color,
                        &background_entity,
                        background_color,
                    );
                }
            }
        }

        map_context.finalize();
    }
}

/// Calculates the current frame after `elapsed_time_64` seconds
/// Adjusts `tile` accordingly.
/// returns `true` if `tile.current_index` changed
/// otherwise returns `false`
fn calc_current_frame(tile: &mut AnimatedTile, elapsed_time_64: f64) -> bool {
    tile.elapsed_time += elapsed_time_64;
    let current_index = (tile.frames_per_second * tile.elapsed_time
        % tile.foreground_tiles.len() as f64)
        .floor() as usize;
    if tile.current_index != current_index {
        tile.current_index = current_index;
        true
    } else {
        false
    }
}
