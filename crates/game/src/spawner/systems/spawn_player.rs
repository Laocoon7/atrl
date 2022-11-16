use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    info!("Spawning player!");

    commands.spawn(PlayerBundle {
        player: Player,

        world_position: WorldPosition(IVec3::ZERO),

        movement_component: Movement {
            movement_types: (MovementType::Walk as u8) | (MovementType::Swim as u8),
        },
        vision_component: Vision { vision_types: (VisionType::Colored as u8) },

        actor: ActorBundle {
            name: Name::new("Bob the Builder"),
            position: LocalPosition(IVec2::new((GRID_WIDTH / 2) as i32, (GRID_HEIGHT / 2) as i32)),
            health: Health::new(10, 10),
            ai: AIComponent::human(),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::PINK,
                    index: from_cp437('@'),
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                texture_atlas: texture_assets.terminal8x8_atlas.clone(),
                transform: Transform::from_xyz(
                    (GRID_WIDTH / 2) as f32 + 0.5,
                    (GRID_HEIGHT / 2) as f32 + 0.5,
                    f32::from(MapLayer::Player), /* TODO: We need to have set Z values for each
                                                  * different layer */
                ),
                ..Default::default()
            },
        },
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::default_input_map(),
            ..default()
        },
    });
}
