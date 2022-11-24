use crate::prelude::*;
pub fn spawn_player(mut commands: Commands, tilesets: Tilesets, mut map_manager: ResMut<MapManager>) {
    let world_position = IVec3::ZERO;
    let Some(tileset) = tilesets.get_by_id(TILESET_ACTORS_ID) else {
        // crashing here, may make it hard to chase down other issues?
        error!("Couldn't find tilemap_id: {:?}. Refusing to spawn player.", TILESET_ACTORS_ID);
        return;
    };
    let Some(map) = map_manager.get_current_map_mut() else {
        error!("Couldn't find a current map. Refusing to spawn player.");
        return;
    };
    let entity = commands.spawn_empty().id();
    let local_position = UVec2::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
    let movement_type = MovementType::Walk.as_u8() | MovementType::Swim.as_u8();
    if !map.try_add_actor(local_position, entity, movement_type) {
        error!("Couldn't place player actor at {:?}", local_position);
        commands.entity(entity).despawn();
        return;
    } else {
        info!("Player spawned at {:?}", local_position);
    }
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
                    index: TILE_ACTOR_OGRE_ID,
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                texture_atlas: tileset.atlas().clone(),
                transform: Transform::from_xyz(
                    (GRID_WIDTH / 2) as f32 + 0.5,
                    (GRID_HEIGHT / 2) as f32 + 0.5,
                    f32::from(MapLayer::Player),
                ),
                ..Default::default()
            },

            fov: FieldOfView(16),
            vision_component: Vision(VisionType::Normal.as_u8()),
            movement_component: Movement(movement_type),
            target_visualizer: TargetVisualizer::default(),
        },
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::default_input_map(),
            ..default()
        },
    });
}
