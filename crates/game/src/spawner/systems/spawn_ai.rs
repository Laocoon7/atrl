use crate::prelude::*;

pub fn spawn_ai(tilesets: Tilesets, mut commands: Commands, game_context: Res<GameContext>) {
    println!("Spawning AI!");

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

    // let chase_and_attack = Steps::build().step(ChaseActor::default());

    // Build the thinker
    let thinker = Thinker::build()
        .label("RandomThinker")
        // We don't do anything unless we're thirsty enough.
        .picker(FirstToScore { threshold: 0.8 })
        // .when(WinningScorer::build(1.0).push(CanSeePlayer::default()), chase_and_attack)
        .otherwise(Wander::default());

    commands.spawn((
        ActorBundle {
            ai: AIComponent::aggressive(),
            name: Name::new("Gary the Destroyer"),
            position: WorldPosition(world_position),
            health: Health::new(5, 5),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::RED,
                    index: 4,
                    // index: from_cp437('G'),
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                texture_atlas: tileset.atlas().clone(),
                transform: Transform::from_xyz(
                    (GRID_WIDTH / 3) as f32 + 0.5,
                    (GRID_HEIGHT / 3) as f32 + 0.5,
                    f32::from(MapLayer::Player), /* TODO: We need to have set Z values for each
                                                  * different layer */
                ),
                ..default()
            },
            vision_component: Vision(VisionType::Colored.into()),
            movement_component: Movement(MovementType::Walk.into()),
        },
        thinker,
    ));
}
