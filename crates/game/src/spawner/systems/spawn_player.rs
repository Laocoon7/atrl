use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, tilesets: Tilesets) {
    println!("Spawning player!");

    let world_position = IVec3::ZERO;

    let tileset = tilesets
        .get_by_id(TILESET_ACTORS_OGRE_ID)
        .unwrap_or_else(|| panic!("couldn't find tilemap_id: {:?}", *TILESET_ACTORS_OGRE_ID));

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
                    index: 0,
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

            fov: FieldOfView(8),
            vision_component: Vision(VisionType::Normal),
            movement_component: Movement(MovementType::Walk.as_u8() | (MovementType::Swim.as_u8())),
        },
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::default_input_map(),
            ..default()
        },
    });
}
