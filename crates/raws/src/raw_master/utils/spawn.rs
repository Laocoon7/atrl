use crate::prelude::*;

impl RawMaster {
    // pub fn spawn_mob_from_raw(
    //     &self,
    //     commands: &mut Commands,
    //     tileset: &Tileset,
    //     map_manager: &mut MapManager,
    //     q_blocks_movement: &Query<&BlocksMovement>,
    //     position: Position,
    //     mob_type: MobType,
    // ) -> Option<Entity> {
    //     let raws = self.get_raws();

    //     let RawActor {
    //         name,
    //         stats,
    //         vision,
    //         movement,
    //         vision_range,
    //     } = raws.get_mob(mob_type)?;

    //     let movement = MovementType::from_vec(movement.to_vec());

    //     let mob = commands.spawn_empty().id();
    //     if !map_manager.add_actor(mob, position, movement, q_blocks_movement) {
    //         error!("Couldn't place mob actor at {:?}", position.gridpoint());
    //         commands.entity(mob).despawn();
    //         return None;
    //     } else {
    //         info!("Mob spawned at {:?}", position.gridpoint());
    //     }

    //     Some(
    //         commands
    //             .entity(mob)
    //             .insert(ActorBundle {
    //                 mob: Mob,
    //                 position,
    //                 name: Name::new(name.clone()),
    //                 ai: AIComponent::aggressive(),
    //                 fov: FieldOfView(*vision_range),
    //                 health: Health::full(stats.max_hp),
    //                 movement_component: Movement(movement),
    //                 target_visualizer: TargetVisualizer::default(),
    //                 vision_component: Vision(VisionType::from_vec(vision.to_vec())),

    //                 sprite: SpriteSheetBundle {
    //                     texture_atlas: tileset.texture_atlas.clone(),
    //                     transform: Transform::from_xyz(
    //                         position.worldpoint().x,
    //                         position.worldpoint().y,
    //                         position.layer() as f32,
    //                     ),
    //                     ..Default::default()
    //                 },
    //             })
    //             .id(),
    //     )
    // }

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
