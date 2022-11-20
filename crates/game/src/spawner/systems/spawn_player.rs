use crate::prelude::*;

pub fn spawn_player(tilesets: Tilesets, mut commands: Commands, game_context: Res<GameContext>) {
    println!("Spawning player!");

    let world_position = IVec3::ZERO;
    let rng = &mut game_context.get_random();

    // TODO: check deserialize map from world_position
    let map_noise = rng.noise.get(world_position.x, world_position.y, world_position.z);
    let map_noise = (map_noise + 1.0) * 0.5; // TODO: Verify noise.get() returns (-1, 1)

    let tileset_count = tilesets.len() as f64 - 1.0;
    let tileset_selection = (tileset_count * map_noise).round() as u8;
    let tileset = tilesets
        .get_by_id(&tileset_selection)
        .unwrap_or_else(|| panic!("couldn't find tilemap_id: {:?}", tileset_selection));

    commands.spawn(PlayerBundle {
        player: Player,

        actor: ActorBundle {
            name: Name::new("Bob the Builder"),
            position: WorldPosition(world_position),
            health: Health::new(10, 10),
            ai: AIComponent::human(),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::YELLOW,
                    index: 4,
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                texture_atlas: tileset.atlas().clone(),
                transform: Transform::from_xyz(
                    (GRID_WIDTH / 2) as f32 + 0.5,
                    (GRID_HEIGHT / 2) as f32 + 0.5,
                    f32::from(MapLayer::Player), /* TODO: We need to have set Z values for each
                                                  * different layer */
                ),
                ..Default::default()
            },

            fov: FieldOfView(8),
            vision_component: Vision(VisionType::Normal.as_u8()),
            movement_component: Movement(MovementType::Walk.as_u8() | (MovementType::Swim as u8)),
        },
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::default_input_map(),
            ..default()
        },
    });
}
