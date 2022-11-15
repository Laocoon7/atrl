use crate::prelude::*;

pub(crate) fn gen_tilemaps_for_maps(
    mut commands: Commands,
    array_texture_loader: Res<ArrayTextureLoader>,
    tilesets: Tilesets,
    mut q_map: Query<&mut Map, Without<TilemapType>>,
) {
    // Get Maps without Tilemaps
    for mut map in q_map.iter_mut() {
        if map.terrain_storage_entity.is_some()
            || map.feature_storage_entity.is_some()
            || map.item_storage_entity.is_some()
        {
            continue;
        }

        // create the Tilemaps for each layer
        if let Some(tileset) = tilesets.get_by_id(&map.terrain_tileset_id) {
            let tilemap_entity = commands.spawn_empty().id();
            map.terrain_storage_entity = Some(tilemap_entity);
            add_tilemap_to_entity(
                &mut commands,
                tilemap_entity,
                map.size,
                f32::from(MapLayer::Terrain),
                tileset,
                &array_texture_loader,
            );
        } else {
            error!("Tileset not found! TilesetId: {}", &map.terrain_tileset_id);
        }

        if let Some(tileset) = tilesets.get_by_id(&map.feature_tileset_id) {
            let tilemap_entity = commands.spawn_empty().id();
            map.feature_storage_entity = Some(tilemap_entity);
            add_tilemap_to_entity(
                &mut commands,
                tilemap_entity,
                map.size,
                f32::from(MapLayer::Features),
                tileset,
                &array_texture_loader,
            );
        } else {
            error!("Tileset not found! TilesetId: {}", &map.feature_tileset_id);
        }

        if let Some(tileset) = tilesets.get_by_id(&map.item_tileset_id) {
            let tilemap_entity = commands.spawn_empty().id();
            map.item_storage_entity = Some(tilemap_entity);
            add_tilemap_to_entity(
                &mut commands,
                tilemap_entity,
                map.size,
                f32::from(MapLayer::Items),
                tileset,
                &array_texture_loader,
            );
        } else {
            error!("Tileset not found! TilesetId: {}", &map.item_tileset_id);
        }
    }
}
