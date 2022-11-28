use std::time::Duration;

use crate::prelude::*;

pub fn player_input(
    state: Res<TurnState>,
    mut commands: Commands,
    mut move_events: EventWriter<WantsToMove>,
    mut query: Query<(Entity, &Transform, &ActionState<PlayerAction>), With<Player>>,
    mut action_queue: ResMut<ActionQueue>,
) {
    for (player, position, action_state) in query.iter_mut() {
        // Actions
        if action_state.just_pressed(PlayerAction::Wait) {
            info!("Player waited");
            action_queue.add_action(ActionType::Wait);
            // state.set_next(&mut commands);
        }

        // Movement
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction) ||
                (action_state.pressed(input_direction) &&
                    action_state.current_duration(input_direction) > Duration::from_millis(500))
            {
                if let Some(direction) = input_direction.direction() {
                    let last_position = position.get();
                    let destination = last_position + direction.coord();
                    action_queue.add_action(ActionType::Movement(destination));
                    // move_events.send(WantsToMove(player, destination));
                    // state.set_next(&mut commands);
                }
            }
        }
    }
}
