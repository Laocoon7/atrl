use crate::prelude::*;
/// TODO:
///
/// the function has a cognitive complexity of (28/25)
/// you could split it up into multiple smaller functions
///
/// https://rust-lang.github.io/rust-clippy/master/index.html#cognitive_complexity
pub fn update_tilemaps(
    mut map_manager: ResMut<MapManager>,
    q_storage: Query<&TileStorage>,
    mut q_tiles: Query<&mut TileTextureIndex>,
) {
    if let Some(map) = map_manager.get_current_map_mut() {
        if !map.update_all && map.update_tiles.is_empty() {
            return;
        }

        // Get storages
        let terrain_storage = match q_storage.get(map.terrain_layer_entity) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                return;
            },
        };

        let feature_storage = match q_storage.get(map.feature_layer_entity) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                return;
            },
        };

        let mut check_next = Vec::new();
        if map.update_all {
            for y in 0..map.size.height() {
                for x in 0..map.size.width() {
                    let tile_pos = TilePos::new(x, y);
                    if let Some(entity) = terrain_storage.get(&tile_pos) {
                        if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                            let index = (*map.terrain_types.get_unchecked((x, y))).into();
                            tile_texture_index.0 = index;
                        } else {
                            check_next.push(UVec2::new(x, y));
                        }
                    }

                    if let Some(entity) = feature_storage.get(&tile_pos) {
                        if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                            let index = (*map.feature_types.get_unchecked((x, y))).into();
                            tile_texture_index.0 = index;
                        } else {
                            check_next.push(UVec2::new(x, y));
                        }
                    }
                }
            }

            for v in check_next.into_iter() {
                map.update_tiles.insert(v);
            }
            map.update_all = false;
        } else {
            let mut positions = std::mem::take(&mut map.update_tiles);
            for position in positions.drain() {
                let tile_pos = TilePos::new(position.x, position.y);
                if let Some(entity) = terrain_storage.get(&tile_pos) {
                    if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                        tile_texture_index.0 = (*map.terrain_types.get_unchecked(position)).into();
                    } else {
                        map.update_tiles.insert(UVec2::new(tile_pos.x, tile_pos.y));
                    }
                }

                if let Some(entity) = feature_storage.get(&tile_pos) {
                    if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                        tile_texture_index.0 = (*map.feature_types.get_unchecked(position)).into();
                    } else {
                        map.update_tiles.insert(UVec2::new(tile_pos.x, tile_pos.y));
                    }
                }
            }
        }
    }
}
