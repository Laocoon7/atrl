use crate::prelude::*;

pub fn create_tilemap(
    commands: &mut Commands,
    size: impl Size2d,
    tileset: &Tileset,
    array_texture_loader: &Res<ArrayTextureLoader>,
) {
    let tilemap_size = TilemapSize { x: size.width(), y: size.height() };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    for y in 0..tilemap_size.y {
        for x in 0..tilemap_size.x {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(0),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: tileset.tile_size().x, y: tileset.tile_size().y };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        tile_size,
        grid_size,
        map_type,
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tileset.texture().clone()),
        transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    array_texture_loader.add(TilemapArrayTexture {
        texture: TilemapTexture::Single(tileset.texture().clone()),
        tile_size,
        ..Default::default()
    });
}
