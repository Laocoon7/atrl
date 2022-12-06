use std::time::Duration;

use crate::prelude::*;

pub struct PlayerTimer {
    pub input_delay_timer: Timer,
    pub unsafe_delay_timer: Option<Timer>,
}

impl FromWorld for PlayerTimer {
    fn from_world(world: &mut World) -> Self {
        if let Some(game_settings) = world.get_resource::<GameSettings>() {
            Self {
                input_delay_timer: Timer::new(game_settings.repeat_duration(), TimerMode::Once),
                unsafe_delay_timer: None,
            }
        } else {
            error!("PlayerTimer resource needs to be inserted after GameSettings ressource.");
            Self::new()
        }
    }
}

impl PlayerTimer {
    fn new() -> Self {
        Self {
            input_delay_timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
            unsafe_delay_timer: None,
        }
    }
}

#[derive(Resource, Default)]
pub struct UnsafeInput;

pub fn player_input(
    mut commands: Commands,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
    mut timer: Local<PlayerTimer>,
    mut action_queue: ResMut<ActionQueue>,
    mut query: Query<&ActionState<PlayerAction>>,
    check_safe: Option<Res<UnsafeInput>>,
) {
    // If an event happens which the player should pay attention to,
    // UnsafeInput should be inserted as a resource.
    if check_safe.is_some() {
        // stop all actions
        action_queue.clear_actions();

        // If we already have a timer going
        if let Some(unsafe_timer) = &mut timer.unsafe_delay_timer {
            // tick it
            unsafe_timer.tick(time.delta());

            // if the timer is finished
            if unsafe_timer.finished() {
                // clear the UnsafeInput resource
                commands.remove_resource::<UnsafeInput>();
                // remove the timer
                timer.unsafe_delay_timer = None;
            }
        } else {
            // start a new timer.
            timer.unsafe_delay_timer = Some(Timer::new(game_settings.unsafe_duration(), TimerMode::Once));
        }

        // No input this frame!
        return;
    }

    // Tick timer until duration is met.
    if !timer.input_delay_timer.finished() {
        timer.input_delay_timer.tick(time.delta());
    }

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
                    action_state.current_duration(input_direction) > game_settings.pressed_duration()) &&
                    timer.input_delay_timer.finished()
            {
                if let Some(direction) = input_direction.direction() {
                    timer.input_delay_timer.reset();
                    // Input is taken as a direction the player wants to move,
                    // We can apply that to the current position when the player
                    // *ACTUALLY* gets to move.
                    action_queue.add_action(ActionType::MovementDelta(direction.coord()));

                    println!();
                    info!("Player gave input: MOVE");
                }
            }
        }
    }
}
