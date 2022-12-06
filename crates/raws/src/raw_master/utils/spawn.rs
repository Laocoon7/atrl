use crate::prelude::*;

impl RawMaster {
    pub fn spawn_mob_from_raw(
        &self,
        commands: &mut Commands,
        texture_atlas: &Handle<TextureAtlas>,
        map_manager: &mut MapManager,
        q_blocks_movement: &Query<&BlocksMovement>,
        position: Position,
        mob_key: &str,
    ) -> Option<Entity> {
        let RawActor {
            name,
            vision,
            movement,
            vision_range,
            stats: RawStats { max_hp },
        } = self.get_mob(mob_key)?;

        let movement = MovementType::from_vec(movement.to_vec());

        let mob = commands.spawn_empty().id();
        if !map_manager.add_actor(mob, position, movement, q_blocks_movement) {
            error!("Couldn't place mob actor at {:?}", position.gridpoint());
            commands.entity(mob).despawn();
            return None;
        } else {
            info!("Mob spawned at {:?}", position.gridpoint());
            map_manager.add_actor(mob, position, movement, q_blocks_movement);
        }

        // Build the thinker
        let thinker = Thinker::build()
            .picker(FirstToScore { threshold: 0.8 })
            .when(
                // WinningScorer::build(1.0).push(CanSeePlayer::default()),
                CanSeePlayer::default(),
                Steps::build().step(ChaseActor::default()).step(AttackActor::default()),
            )
            .otherwise(Wander::default());

        Some(
            commands
                .spawn((
                    ActorBundle {
                        mob: Mob,
                        position,
                        ai: AIComponent::aggressive(),
                        name: Name::new(name.to_string()),
                        health: Health::full(*max_hp),
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

                        fov: FieldOfView(*vision_range),
                        vision_component: Vision::from(vision.to_vec()),
                        movement_component: Movement(movement),
                        target_visualizer: TargetVisualizer::default(),
                    },
                    thinker,
                ))
                .id(),
        )
    }

    pub fn spawn_player_from_raw(
        &self,
        commands: &mut Commands,
        tileset: &Tileset,
        map_manager: &mut MapManager,
        q_blocks_movement: &Query<&BlocksMovement>,
        position: Position,
    ) -> Option<Entity> {
        let raws = self.get_raws();

        let RawActor {
            name,
            stats,
            vision,
            movement,
            vision_range,
        } = &raws.player;

        let movement = MovementType::from_vec(movement.to_vec());

        let player = commands.spawn_empty().id();
        if !map_manager.add_actor(player, position, movement, q_blocks_movement) {
            error!("Couldn't place player actor at {:?}", position.gridpoint());
            commands.entity(player).despawn();
            return None;
        } else {
            info!("Player spawned at {:?}", position.gridpoint());
            map_manager.add_actor(player, position, movement, q_blocks_movement);
        }

        Some(
            commands
                .entity(player)
                .insert(PlayerBundle {
                    actor: ActorBundle {
                        mob: Mob,
                        position,
                        ai: AIComponent::player(),
                        name: Name::new(name.clone()),
                        fov: FieldOfView(*vision_range),
                        health: Health::full(stats.max_hp),
                        vision_component: Vision(VisionType::from_vec(vision.to_vec())),
                        movement_component: Movement(movement),
                        target_visualizer: TargetVisualizer::default(),

                        sprite: SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                color: Color::YELLOW,
                                index: TILE_ACTOR_OGRE_ID,
                                custom_size: Some(Vec2::ONE),
                                ..Default::default()
                            },
                            texture_atlas: tileset.atlas().clone(),
                            transform: Transform::from_translation(position.translation()),
                            ..Default::default()
                        },
                    },
                    input_manager: InputManagerBundle {
                        input_map: PlayerBundle::default_input_map(),
                        ..default()
                    },
                })
                .id(),
        )
    }
}
