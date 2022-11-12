use crate::prelude::*;

#[derive(Bundle)]
pub struct ForegroundTileBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub foreground_tile: ForegroundTile,
}

impl ForegroundTileBundle {
    pub fn from_foreground_tile(foreground_tile: ForegroundTile, position: Vec3) -> Self {
        Self {
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
            foreground_tile,
        }
    }
}
