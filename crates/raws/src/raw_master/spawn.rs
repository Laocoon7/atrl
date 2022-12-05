use crate::prelude::*;

impl RawMaster {
    pub fn spawn_player_from_raw(
        &self,
        commands: &mut Commands,
        tileset: &Tileset,
        map_manager: &mut MapManager,
        q_blocks_movement: &Query<&BlocksMovement>,
        position: &Position,
    ) -> Option<Entity> {
        let raws = self.get_raws();

        let RawPlayer {
            name,
            stats,
            vision,
            movement,
            vision_range,
        } = raws.player.as_ref().expect("Player not loaded");

        let movement = MovementType::from_vec(movement.to_vec());

        let player = commands.spawn_empty().id();
        if !map_manager.add_actor(player, *position, movement, q_blocks_movement) {
            error!("Couldn't place player actor at {:?}", position.gridpoint());
            commands.entity(player).despawn();
            return None;
        } else {
            info!("Player spawned at {:?}", position.gridpoint());
        }

        println!(
            "Player spawned at {:?}",
            VisionType::from_vec(vision.to_vec())
        );
        Some(
            commands
                .entity(player)
                .insert(PlayerBundle {
                    actor: ActorBundle {
                        mob: Mob,
                        position: *position,
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
