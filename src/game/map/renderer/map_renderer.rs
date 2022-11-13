use crate::game::prelude::internal::*;
use crate::prelude::*;

#[derive(Resource)]
pub struct MapRenderer {
    pub size: UVec2,
    pub terrain_entities: Grid<Entity>,
    pub terrain_background_entities: Grid<Entity>,
    pub feature_entities: Grid<Entity>,
    pub feature_background_entities: Grid<Entity>,
    pub item_entities: Grid<Entity>,
    pub item_background_entities: Grid<Entity>,
}

#[allow(dead_code)]
impl MapRenderer {
    pub fn build(size: impl Size2d, game_assets: &Res<TextureAssets>, commands: &mut Commands) {
        let terrain_entities = Grid::new_fn(size, |index| {
            commands
                .spawn(RenderTileBundle {
                    sprite_sheet_bundle: SpriteSheetBundle {
                        texture_atlas: game_assets.terminal8x8_atlas.clone(),
                        sprite: TextureAtlasSprite {
                            index: 0,
                            color: Color::WHITE,
                            custom_size: Some(Vec2::ONE),
                            anchor: bevy::sprite::Anchor::Center,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(index.x as f32, index.y as f32, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id()
        });
        let feature_entities = Grid::new_fn(size, |index| {
            commands
                .spawn(RenderTileBundle {
                    sprite_sheet_bundle: SpriteSheetBundle {
                        texture_atlas: game_assets.terminal8x8_atlas.clone(),
                        sprite: TextureAtlasSprite {
                            index: 0,
                            color: Color::WHITE,
                            custom_size: Some(Vec2::ONE),
                            anchor: bevy::sprite::Anchor::Center,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(index.x as f32, index.y as f32, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id()
        });
        let item_entities = Grid::new_fn(size, |index| {
            commands
                .spawn(RenderTileBundle {
                    sprite_sheet_bundle: SpriteSheetBundle {
                        texture_atlas: game_assets.terminal8x8_atlas.clone(),
                        sprite: TextureAtlasSprite {
                            index: 0,
                            color: Color::WHITE,
                            custom_size: Some(Vec2::ONE),
                            anchor: bevy::sprite::Anchor::Center,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(index.x as f32, index.y as f32, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id()
        });

        let terrain_background_entities = Grid::new_fn(size, |index| {
            commands
                .spawn(RenderTileBundleSprite {
                    sprite_bundle: SpriteBundle {
                        texture: game_assets.white_pixel.clone(),
                        sprite: Sprite {
                            color: Color::BLACK,
                            custom_size: Some(Vec2::ONE),
                            anchor: bevy::sprite::Anchor::Center,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(index.x as f32, index.y as f32, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id()
        });
        let feature_background_entities = Grid::new_fn(size, |index| {
            commands
                .spawn(RenderTileBundleSprite {
                    sprite_bundle: SpriteBundle {
                        texture: game_assets.white_pixel.clone(),
                        sprite: Sprite {
                            color: Color::NONE,
                            custom_size: Some(Vec2::ONE),
                            anchor: bevy::sprite::Anchor::Center,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(index.x as f32, index.y as f32, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id()
        });
        let item_background_entities = Grid::new_fn(size, |index| {
            commands
                .spawn(RenderTileBundleSprite {
                    sprite_bundle: SpriteBundle {
                        texture: game_assets.white_pixel.clone(),
                        sprite: Sprite {
                            color: Color::NONE,
                            custom_size: Some(Vec2::ONE),
                            anchor: bevy::sprite::Anchor::Center,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(index.x as f32, index.y as f32, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id()
        });

        commands.insert_resource(Self {
            size: size.as_uvec2(),
            terrain_entities,
            terrain_background_entities,
            feature_entities,
            feature_background_entities,
            item_entities,
            item_background_entities,
        });
    }

    #[inline(always)]
    pub const fn get_context(&self) -> RenderContext { RenderContext::new(self) }

    pub fn get_entity(&self, layer: MapLayer, index: impl Point2d) -> Option<Entity> {
        match layer {
            MapLayer::Terrain => self.terrain_entities.get(index).copied(),
            MapLayer::Features => self.feature_entities.get(index).copied(),
            MapLayer::Items => self.item_entities.get(index).copied(),
        }
    }

    pub fn get_background_entity(&self, layer: MapLayer, index: impl Point2d) -> Option<Entity> {
        match layer {
            MapLayer::Terrain => self.terrain_background_entities.get(index).copied(),
            MapLayer::Features => self.feature_background_entities.get(index).copied(),
            MapLayer::Items => self.item_background_entities.get(index).copied(),
        }
    }
}
