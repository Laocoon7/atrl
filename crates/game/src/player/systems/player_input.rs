use std::time::Duration;

use crate::prelude::*;

const REPEAT_DURATION: Duration = Duration::from_millis(100);
const PRESSED_DURATION: Duration = Duration::from_millis(500);

#[derive(Deref, DerefMut)]
pub struct PlayerTimer(pub Timer);

impl Default for PlayerTimer {
    fn default() -> Self { Self(Timer::new(REPEAT_DURATION, TimerMode::Once)) }
}

pub fn player_input(
    time: Res<Time>,
    mut timer: Local<PlayerTimer>,
    mut action_queue: ResMut<ActionQueue>,
    mut query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    // Tick timer until duration is met.
    if !timer.finished() {
        timer.tick(time.delta());
    }

    for action_state in query.iter_mut() {
        // Actions
        if action_state.just_pressed(PlayerAction::Wait) {
            action_queue.add_action(ActionType::Wait);
        }

        // Movement
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction) ||
                (action_state.pressed(input_direction) &&
                    action_state.current_duration(input_direction) > PRESSED_DURATION) &&
                    timer.finished()
            {
                if let Some(direction) = input_direction.direction() {
                    timer.reset();
                    action_queue.add_action(ActionType::MovementDelta(direction.coord()));

                    println!();
                    info!("Player turn end");
                }
            }
        }
    }
}

pub fn draw_shape(
    mut commands: Commands,
    q_player: Query<&Position, With<Player>>,
    tilesets: Tilesets,
    mut has_ran: Local<bool>,
) {
    if *has_ran {
        return;
    }
    *has_ran = true;

    if let Some(tileset) = tilesets.get_by_id(&TILESET_UI_ID) {
        for position in q_player.iter() {
            let circle = Circle::new(position.gridpoint(), 5u32);
            for point in circle.get_points() {
                commands.spawn(SpriteSheetBundle {
                    texture_atlas: tileset.atlas().clone(),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::ONE),
                        index: TILE_UI_CURSOR_GREEN_ID as usize,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        point.x as f32 + 0.5,
                        point.y as f32 + 0.5,
                        f32::from(MapLayer::UI),
                    )),
                    ..Default::default()
                });
            }
        }
    }
}
