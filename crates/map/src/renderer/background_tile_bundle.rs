use crate::prelude::*;

#[derive(Bundle)]
pub struct BackgroundTileBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub background_tile: BackgroundTile,
}

impl BackgroundTileBundle {
    pub fn from_color(texture: &Handle<Image>, color: Color, position: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::ONE),
                    anchor: Center,
                    ..Default::default()
                },
                transform: Transform::from_translation(position),
                texture: texture.clone(),
                ..Default::default()
            },
            background_tile: BackgroundTile {},
        }
    }
}
