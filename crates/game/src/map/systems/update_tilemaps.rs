use crate::prelude::*;

pub(crate) fn update_tilemaps(
    mut q_map: Query<&mut Map>,
    q_storage: Query<&TileStorage>,
    mut q_tiles: Query<&mut TileTextureIndex>,
) {
    for mut map in q_map.iter_mut() {
        if !map.update_all && map.update_tiles.is_empty() {
            continue;
        }

        // Get storages
        let terrain_storage = match q_storage.get(map.terrain_layer_entity) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };

        let feature_storage = match q_storage.get(map.feature_layer_entity) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };

        let item_storage = match q_storage.get(map.item_layer_entity) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                continue;
            }
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

                    if let Some(entity) = item_storage.get(&tile_pos) {
                        if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                            // TODO: Display Items
                            // sort through items to decide which to show
                            if let Some((index, _)) =
                                map.item_types.get_unchecked((x, y)).iter().enumerate().next()
                            {
                                tile_texture_index.0 = index as u32;
                            }
                        }
                    }
                }
            }

            for v in check_next.drain(..) {
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

                if let Some(entity) = item_storage.get(&tile_pos) {
                    if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                        // TODO: Display Items
                        // sort through items to decide which to show
                        if let Some((index, _)) =
                            map.item_types.get_unchecked(position).iter().enumerate().next()
                        {
                            tile_texture_index.0 = index as u32;
                        }
                    }
                }
            }
        }
    }
}
