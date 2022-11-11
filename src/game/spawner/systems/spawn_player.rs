use crate::{
    app_settings::{GRID_HEIGHT, GRID_WIDTH},
    game::prelude::*,
};
use atrl_engine::bevy::prelude::*;
use leafwing_input_manager::InputManagerBundle;

pub fn spawn_player(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    info!("Spawning player!");

    commands.spawn_bundle(PlayerBundle {
        player: Player,
        actor: ActorBundle {
            name: Name::new("Bob the Builder"),
            position: LocalPosition::new(IVec2::new(
                (GRID_WIDTH / 2) as i32,
                (GRID_HEIGHT / 2) as i32,
            )),
            health: Health::new(10, 10),
            ai: AIComponent::human(),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::PINK,
                    index: from_cp437('@'),
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                texture_atlas: texture_assets.terminal8x8_atlas.clone(),
                transform: Transform::from_xyz(
                    (GRID_WIDTH / 2) as f32,
                    (GRID_HEIGHT / 2) as f32,
                    10.0, // TODO: We need to have set Z values for each different layer
                ),
                ..Default::default()
            },
        },
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::default_input_map(),
            ..default()
        },
    });
}
