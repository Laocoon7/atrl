use crate::prelude::*;

const MAX_AI_ACTORS: u32 = 2;

pub fn spawn_ai(
    tilesets: Tilesets,
    mut commands: Commands,
    state: Res<CurrentGameState>,
    mut map_manager: ResMut<MapManager>,
    mut turn_manager: ResMut<TurnManager>,
) {
    let world_position = IVec3::ZERO;

    let Some(tileset) = tilesets.get_by_id(&TILESET_ACTORS_ID) else {
        // let's not crash the program just because we can't spawn an ai.
        error!("Couldn't find tilemap_id: {:?}. Refusing to spawn ai.", TILESET_ACTORS_ID);
        return;
    };

    let Some(map) = map_manager.get_current_map_mut() else {
        error!("Couldn't find a current map. Refusing to spawn ai.");
        return;
    };

    let mut actor_count = 0;
    let movement_type = MovementType::Walk;
    let vision_type = VisionType::Normal;
    for y in 0..MAX_AI_ACTORS {
        if actor_count >= MAX_AI_ACTORS {
            break;
        }

        for x in 0..MAX_AI_ACTORS {
            if actor_count >= MAX_AI_ACTORS {
                break;
            }

            let local_position = UVec2::new(GRID_WIDTH / 3 + x, GRID_HEIGHT / 3 + y);
            if map.can_place_actor(local_position, movement_type.as_u8()) {
                let ai_entity = spawn_ai_at(
                    &mut commands,
                    tileset.atlas(),
                    format!("Gary ({})", actor_count).as_str(),
                    local_position,
                    world_position,
                    vision_type,
                    movement_type,
                );
                turn_manager.add_entity(ai_entity);
                map.try_add_actor(local_position, ai_entity, movement_type.as_u8());
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
    local_position: UVec2,
    world_position: IVec3,
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

    let position = Position::new(
        WorldPosition::new(world_position.x, world_position.y, world_position.z),
        LocalPosition::new(local_position.x, local_position.y, MapLayer::Actors as u32),
    );

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
