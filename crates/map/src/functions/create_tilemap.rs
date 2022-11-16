use crate::prelude::*;

pub fn create_tilemap<ZLevel: Into<f32>>(
    commands: &mut Commands,
    size: impl Size2d,
    z_level: ZLevel,
    tileset: &Tileset,
    array_texture_loader: &Res<ArrayTextureLoader>,
) {
    let tilemap_entity = commands.spawn_empty().id();
    add_tilemap_to_entity(commands, tilemap_entity, size, z_level, tileset, array_texture_loader);
}

pub fn add_tilemap_to_entity<ZLevel: Into<f32>>(
    commands: &mut Commands,
    tilemap_entity: Entity,
    size: impl Size2d,
    z_level: ZLevel,
    tileset: &Tileset,
    array_texture_loader: &Res<ArrayTextureLoader>,
) {
    let tilemap_size = TilemapSize { x: size.width(), y: size.height() };
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

    // scale the map by the tile_size so that
    // 1 Transform unit = 1 tile.
    // shift the map by 1/2 a tile in both x / y
    let transform = Transform {
        translation: Vec3 { x: 0.5, y: 0.5, z: 0.0 },
        scale: Vec3 { x: 1.0 / tile_size.x, y: 1.0 / tile_size.y, z: z_level.into() },
        ..Default::default()
    };

    commands.entity(tilemap_entity).insert(TilemapBundle {
        tile_size,
        grid_size,
        map_type,
        size: tilemap_size,
        storage: tile_storage,
        transform,
        texture: TilemapTexture::Single(tileset.texture().clone()),
        ..Default::default()
    });

    // Optimization texture loader... probably doesn't belong in tilemap creation
    //    array_texture_loader.add(TilemapArrayTexture {
    //        texture: TilemapTexture::Single(tileset.texture().clone()),
    //        tile_size,
    //        ..Default::default()
    //    });
}
