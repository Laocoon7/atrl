use crate::prelude::*;

#[derive(Bundle)]
pub struct AnimatedTileBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub animated_tile: AnimatedTile,
    pub background_tile: BackgroundEntityHolder,
}

impl AnimatedTileBundle {
    pub fn from_animation(
        animated_tile: AnimatedTile,
        background_entity: Entity,
        position: Vec3,
    ) -> Option<Self> {
        animated_tile.get_current_foreground_tile().map(|foreground_tile| Self {
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: foreground_tile.index,
                    color: foreground_tile.color,
                    custom_size: Some(Vec2::ONE),
                    anchor: Center,
                    ..Default::default()
                },
                transform: Transform::from_translation(position),
                texture_atlas: foreground_tile.texture_atlas.clone(),
                ..Default::default()
            },
            animated_tile: animated_tile.clone(),
            background_tile: BackgroundEntityHolder { entity: background_entity },
        })
    }
}