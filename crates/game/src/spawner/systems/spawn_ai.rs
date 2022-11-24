use crate::prelude::*;

pub fn spawn_ai(mut commands: Commands, tilesets: Tilesets, mut map_manager: ResMut<MapManager>) {
    let world_position = IVec3::ZERO;

    let Some(tileset) = tilesets.get_by_id(TILESET_ACTORS_ID) else {
        // let's not crash the program just because we can't spawn an ai.
        error!("Couldn't find tilemap_id: {:?}. Refusing to spawn ai.", TILESET_ACTORS_ID);
        return;
    };

    let Some(map) = map_manager.get_current_map_mut() else {
        error!("Couldn't find a current map. Refusing to spawn ai.");
        return;
    };

    let chase_and_attack = Steps::build().step(ChaseActor::default());

    // Build the thinker
    let thinker = Thinker::build()
        .label("RandomThinker",)
        // We don't do anything unless we're thirsty enough.
        .picker(FirstToScore { threshold: 0.8, },)
        .when(WinningScorer::build(1.0,).push(CanSeePlayer::default(),), chase_and_attack,)
        .otherwise(Wander::default(),);

    let entity = commands.spawn_empty().id();
    let local_position = UVec2::new(GRID_WIDTH / 3, GRID_HEIGHT / 3);
    let movement_type = MovementType::Walk.as_u8();

    if !map.try_add_actor(local_position, entity, movement_type) {
        error!("Couldn't place ai actor at {:?}", local_position);
        commands.entity(entity).despawn();
        return;
    } else {
        info!("AI spawned at {:?}", local_position);
    }

    commands.entity(entity).insert((
        ActorBundle {
            ai: AIComponent::aggressive(),
            name: Name::new("Gary the Destroyer"),
            position: WorldPosition(world_position),
            health: Health::new(5, 5),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::RED,
                    index: TILE_ACTOR_OGRE_ID,
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                texture_atlas: tileset.atlas().clone(),
                transform: Transform::from_xyz(
                    (local_position.x) as f32 + 0.5,
                    (local_position.y) as f32 + 0.5,
                    f32::from(MapLayer::Actors),
                ),
                ..default()
            },

            fov: FieldOfView(8),
            vision_component: Vision(VisionType::Normal.as_u8()),
            movement_component: Movement(movement_type),
            target_visualizer: TargetVisualizer::default(),
        },
        thinker,
    ));
}
