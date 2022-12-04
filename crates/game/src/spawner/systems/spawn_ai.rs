use crate::prelude::*;

const MAX_AI_ACTORS: u32 = 1;

pub fn spawn_ai(
    tilesets: Tilesets,
    mut commands: Commands,
    state: Res<CurrentGameState>,
    mut map_manager: MapManager,
    mut turn_manager: ResMut<TurnManager>,
    q_blocks_movement: Query<&BlocksMovement>,
) {
    let Some(tileset) = tilesets.get_by_id(&TILESET_ACTORS_ID) else {
        // let's not crash the program just because we can't spawn an ai.
        error!("Couldn't find tilemap_id: {:?}. Refusing to spawn ai.", TILESET_ACTORS_ID);
        return;
    };

    let mut actor_count = 0;
    let movement_type = MovementType::Walk;
    let vision_type = VisionType::Normal;
    for y in 0..MAX_AI_ACTORS {
        for x in 0..MAX_AI_ACTORS {
            if actor_count >= MAX_AI_ACTORS {
                break;
            }

            let position = Position::new(
                WorldPosition::new(0, 0, 0),
                LocalPosition::new(
                    GRID_WIDTH / 3 + x,
                    GRID_HEIGHT / 3 + y,
                    MapLayer::Actors as u32,
                ),
            );

            if map_manager.can_place_actor(position, movement_type.as_u8(), &q_blocks_movement) {
                let ai_entity = spawn_ai_at(
                    &mut commands,
                    tileset.atlas(),
                    format!("Gary ({})", actor_count).as_str(),
                    position,
                    vision_type,
                    movement_type,
                );
                turn_manager.add_entity(ai_entity);
                map_manager.add_actor(
                    ai_entity,
                    position,
                    movement_type.as_u8(),
                    &q_blocks_movement,
                );
                actor_count += 1;
            }
        }
    }

    state.set_next(&mut commands);
}

fn spawn_ai_at(
    commands: &mut Commands,
    texture_atlas: &Handle<TextureAtlas>,
    name: &str,
    position: Position,
    vision_type: VisionType,
    movement_type: MovementType,
) -> Entity {
    // let chase_and_attack = Steps::build().step(ChaseActor::default());

    // Build the thinker
    let thinker = Thinker::build()
        .picker(FirstToScore { threshold: 0.8 })
        .when(
            // WinningScorer::build(1.0).push(CanSeePlayer::default()),
            CanSeePlayer::default(),
            Steps::build().step(ChaseActor::default()).step(AttackActor::default()),
        )
        .otherwise(Wander::default());

    commands
        .spawn((
            ActorBundle {
                mob: Mob,
                position,
                ai: AIComponent::aggressive(),
                name: Name::new(name.to_string()),
                health: Health::full(5),
                sprite: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        color: Color::RED,
                        index: TILE_ACTOR_OGRE_ID,
                        custom_size: Some(Vec2::ONE),
                        ..Default::default()
                    },
                    texture_atlas: texture_atlas.clone(),
                    transform: Transform::from_translation(position.translation()),
                    ..default()
                },

                fov: FieldOfView(8),
                vision_component: Vision(vision_type.as_u8()),
                movement_component: Movement(movement_type.as_u8()),
                target_visualizer: TargetVisualizer::default(),
            },
            thinker,
        ))
        .id()
}
