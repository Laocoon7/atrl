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
        let terrain_storage = match map.terrain_storage_entity {
            None => continue,
            Some(entity) => {
                //println!("{:?}", entity);
                match q_storage.get(entity) {
                    Err(e) => {
                        error!("{}", e);
                        continue;
                    }
                    Ok(storage) => storage,
                }
            }
        };

        let feature_storage = match map.feature_storage_entity {
            None => continue,
            Some(entity) => {
                //println!("{:?}", entity);
                match q_storage.get(entity) {
                    Err(e) => {
                        error!("{}", e);
                        continue;
                    }
                    Ok(storage) => storage,
                }
            }
        };

        let item_storage = match map.item_storage_entity {
            None => continue,
            Some(entity) => {
                //println!("{:?}", entity);
                match q_storage.get(entity) {
                    Err(e) => {
                        error!("{}", e);
                        continue;
                    }
                    Ok(storage) => storage,
                }
            }
        };

        if map.update_all {
            for y in 0..map.size.height() {
                for x in 0..map.size.width() {
                    let tile_pos = TilePos::new(x, y);
                    if let Some(entity) = terrain_storage.get(&tile_pos) {
                        if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                            let index = (*map.terrain_types.get_unchecked((x, y))).into();
                            tile_texture_index.0 = index;
                        }
                    }

                    if let Some(entity) = feature_storage.get(&tile_pos) {
                        if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                            let index = (*map.feature_types.get_unchecked((x, y))).into();
                            tile_texture_index.0 = index;
                        }
                    }

                    if let Some(entity) = item_storage.get(&tile_pos) {
                        if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                            // sort through items to decide which to show
                            //let index = (*map.item_types.get_unchecked((x, y))).into();
                            //tile_texture_index.0 = index;
                        }
                    }
                }
            }

            map.update_all = false;
        } else {
            let mut positions = std::mem::take(&mut map.update_tiles);
            for position in positions.drain(..) {
                let tile_pos = TilePos::new(position.x, position.y);
                if let Some(entity) = terrain_storage.get(&tile_pos) {
                    if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                        tile_texture_index.0 = (*map.terrain_types.get_unchecked(position)).into();
                    }
                }

                if let Some(entity) = feature_storage.get(&tile_pos) {
                    if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                        tile_texture_index.0 = (*map.feature_types.get_unchecked(position)).into();
                    }
                }

                if let Some(entity) = item_storage.get(&tile_pos) {
                    if let Ok(mut tile_texture_index) = q_tiles.get_mut(entity) {
                        //tile_texture_index.0 =
                        // (*map.item_types.get_unchecked(position)).into();
                    }
                }
            }
        }
    }
}
