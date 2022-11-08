use crate::prelude::*;

#[derive(Component)]
pub struct Tilemap {
    pub id: u64,
    pub background_tile_entities: Vec<Entity>,
    pub foreground_tile_entities: Vec<Entity>,
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
impl Tilemap {
    pub fn build<Id: Into<u64>, Z: Into<f32>, F: FnMut((u32, u32)) -> TileDefinition>(
        id: Id,
        size: impl Size2d,
        z_level: Z,
        texture_atlas_handle: &Handle<TextureAtlas>,
        commands: &mut Commands,
        f: &mut F,
    ) {
        let z_level = z_level.into();
        let size = size.as_uvec2();
        let width = size.x;
        let height = size.y;

        let capacity = (width * height) as usize;
        let mut background_tile_entities = Vec::with_capacity(capacity);
        let mut foreground_tile_entities = Vec::with_capacity(capacity);

        for y in 0..height {
            for x in 0..width {
                let tilemap_tile = f((x, y));
                let background_tile_entity = commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: tilemap_tile.background_index,
                            color: tilemap_tile.background_color,
                            custom_size: Some(Vec2 { x: 1.0, y: 1.0 }),
                            anchor: bevy::sprite::Anchor::BottomLeft,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(x as f32, y as f32, z_level - 0.5),
                        texture_atlas: texture_atlas_handle.clone(),
                        ..Default::default()
                    })
                    .insert(TilemapTile)
                    .id();
                let foreground_tile_entity = commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: tilemap_tile.foreground_index,
                            color: tilemap_tile.foreground_color,
                            custom_size: Some(Vec2 { x: 1.0, y: 1.0 }),
                            anchor: bevy::sprite::Anchor::BottomLeft,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(x as f32, y as f32, z_level),
                        texture_atlas: texture_atlas_handle.clone(),
                        ..Default::default()
                    })
                    .insert(TilemapTile)
                    .id();

                background_tile_entities.push(background_tile_entity);
                foreground_tile_entities.push(foreground_tile_entity);
            }
        }

        commands.spawn().insert(Self {
            id: id.into(),
            background_tile_entities,
            foreground_tile_entities,
            width,
            height,
        });
    }

    pub fn get_editor(&self) -> TilemapEditor { TilemapEditor::new(&self) }

    pub fn get_entity(&self, x: u32, y: u32) -> Option<Entity> {
        if let Some(index) = self.to_index(x, y) {
            Some(self.background_tile_entities[index].clone())
        } else {
            None
        }
    }

    fn to_index(&self, x: u32, y: u32) -> Option<usize> {
        let index = (self.width * y + x) as usize;
        if index < self.background_tile_entities.len() {
            Some(index)
        } else {
            None
        }
    }
}
