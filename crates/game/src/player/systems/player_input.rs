use crate::prelude::*;

pub fn player_input(
    state: Res<TurnState>,
    mut commands: Commands,
    mut move_events: EventWriter<WantsToMove>,
    mut query: Query<(Entity, &Transform, &ActionState<PlayerAction>), (With<MyTurn>, With<Player>)>,
) {
    for (player, position, action_state) in query.iter_mut() {
        // Actions
        if action_state.just_pressed(PlayerAction::Wait) {
            info!("Player waited");
            state.set_next(&mut commands);
        }

        // Movement
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction) {
                if let Some(direction) = input_direction.direction() {
                    let last_position = position.get();
                    let destination = last_position + direction.coord();
                    move_events.send(WantsToMove(player, destination));
                    state.set_next(&mut commands);
                }
            }
        }
    }
}
