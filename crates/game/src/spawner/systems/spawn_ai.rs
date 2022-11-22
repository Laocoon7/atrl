use crate::prelude::*;

pub fn spawn_ai(mut commands: Commands, tilesets: Tilesets) {
    let world_position = IVec3::ZERO;

    let tileset = tilesets
        .get_by_id(TILESET_ACTORS_OGRE_ID)
        .unwrap_or_else(|| panic!("couldn't find tilemap_id: {:?}", TILESET_ACTORS_OGRE_ID));

    let chase_and_attack = Steps::build().step(ChaseActor::default());

    // Build the thinker
    let thinker = Thinker::build()
        .label("RandomThinker")
        // We don't do anything unless we're thirsty enough.
        .picker(FirstToScore { threshold: 0.8 })
        .when(WinningScorer::build(1.0).push(CanSeePlayer::default()), chase_and_attack)
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
                    index: 0,
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                texture_atlas: tileset.atlas().clone(),
                transform: Transform::from_xyz(
                    (GRID_WIDTH / 3) as f32 + 0.5,
                    (GRID_HEIGHT / 3) as f32 + 0.5,
                    f32::from(MapLayer::Actors),
                ),
                ..default()
            },

            fov: FieldOfView(4),
            vision_component: Vision(VisionType::Normal.as_u8()),
            movement_component: Movement(MovementType::Walk.as_u8()),
        },
        thinker,
    ));
}
