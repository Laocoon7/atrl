use crate::prelude::*;

pub fn spawn_player(
    tilesets: Tilesets,
    mut commands: Commands,
    mut map_manager: ResMut<MapManager>,
    mut turn_manager: ResMut<TurnManager>,
) {
    let Some(tileset) = tilesets.get_by_id(TILESET_ACTORS_ID) else {
        // crashing here, may make it hard to chase down other issues?
        error!("Couldn't find tilemap_id: {:?}. Refusing to spawn player.", TILESET_ACTORS_ID);
        return;
    };

    let Some(map) = map_manager.get_current_map_mut() else {
        error!("Couldn't find a current map. Refusing to spawn player.");
        return;
    };

    let local_position = UVec2::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
    let movement_type = MovementType::Walk.as_u8() | MovementType::Swim.as_u8();

    let player = commands.spawn_empty().id();
    if !map.try_add_actor(local_position, player, movement_type) {
        error!("Couldn't place player actor at {:?}", local_position);
        commands.entity(player).despawn();
        return;
    } else {
        info!("Player spawned at {:?}", local_position);
    }

    let position = Position::new(
        WorldPosition::new(0, 0, 0),
        LocalPosition::new(GRID_WIDTH / 2, GRID_HEIGHT / 2, MapLayer::Player as u32),
    );

    commands.entity(player).insert(PlayerBundle {
        player: Player,

        actor: ActorBundle {
            position,
            mob: Mob,
            name: Name::new("Bob the Builder"),
            health: Health::full(10),
            ai: AIComponent::human(),
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

    turn_manager.add_entity(player);
}
