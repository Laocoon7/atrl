use crate::game::prelude::*;

#[derive(Component)]
pub struct Tilemap {
    pub id: u64,
    pub tile_entities: Vec<Entity>,
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
impl Tilemap {
    pub fn build<F: FnMut((u32, u32)) -> (usize, Color)>(
        id: u64,
        width: u32,
        height: u32,
        z_level: f32,
        texture_atlas_handle: Handle<TextureAtlas>,
        commands: &mut Commands,
        f: &mut F,
    ) {
        let capacity = (width * height) as usize;
        let mut tile_entities = Vec::with_capacity(capacity);

        for y in 0..height {
            for x in 0..width {
                let (tile_index, tile_color) = f((x, y));
                let tile_entity = commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: tile_index,
                            color: tile_color,
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

                tile_entities.push(tile_entity);
            }
        }

        commands.spawn().insert(Self { id, tile_entities, width, height });
    }

    pub fn get_editor(&self) -> TilemapEditor { TilemapEditor::new(&self) }

    pub fn get_entity(&self, x: u32, y: u32) -> Option<Entity> {
        if let Some(index) = self.to_index(x, y) {
            Some(self.tile_entities[index].clone())
        } else {
            None
        }
    }

    fn to_index(&self, x: u32, y: u32) -> Option<usize> {
        let index = (self.width * y + x) as usize;
        if index < self.tile_entities.len() {
            Some(index)
        } else {
            None
        }
    }
}
