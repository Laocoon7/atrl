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
    q_position: Query<&Position>,
    mut timer: Local<PlayerTimer>,
    player_entity: Res<PlayerEntity>,
    mut action_queue: ResMut<ActionQueue>,
    mut query: Query<&ActionState<PlayerAction>>,
) {
    // Tick timer until duration is met.
    if !timer.finished() {
        timer.tick(time.delta());
    }

    let Ok(player_position) = q_position.get(player_entity.current()) else { return; };

    for action_state in query.iter_mut() {
        // Actions
        if action_state.just_pressed(PlayerAction::Wait) {
            action_queue.add_action(ActionType::Wait);
            println!();
            info!("Player gave input: WAIT");
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
                    action_queue.add_action(ActionType::Movement(*player_position + direction));

                    println!();
                    info!("Player gave input: MOVE");
                }
            }
        }
    }
}

pub fn draw_shape(
    tilesets: Tilesets,
    mut commands: Commands,
    mut has_ran: Local<bool>,
    player_entity: Res<PlayerEntity>,
    q_position: Query<&Position>,
) {
    if *has_ran {
        return;
    }

    *has_ran = true;

    if let Some(tileset) = tilesets.get_by_id(&TILESET_UI_ID) {
        let Ok(position) = q_position.get(player_entity.current()) else {
            return;
        };

        // let shape = Line::new(
        //    position.gridpoint(),
        //    position.gridpoint() + UVec2::new(7, 10),
        //);

        let shape = Circle::new(*position, 15u32);

        //let shape = Triangle::new(
        //    position.gridpoint(),
        //    position.gridpoint() + UVec2::new(3, 5),
        //    position.gridpoint() + UVec2::new(3, 2),
        //);

        // let shape = Rectangle::new(
        //     position.gridpoint(),
        //     position.gridpoint() + UVec2::new(10, 5),
        // );

        for point in shape.get_positions() {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: tileset.atlas().clone(),
                sprite: TextureAtlasSprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::ONE),
                    index: TILE_UI_CURSOR_CURSOR_ID,
                    ..Default::default()
                },
                transform: Transform::from_translation(point.translation()),
                ..Default::default()
            });
        }
    }
}
