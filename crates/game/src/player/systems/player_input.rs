use crate::prelude::*;
pub fn player_input(
    state: Res<TurnState>,
    mut commands: Commands,
    mut manager: ResMut<MapManager>,
    mut query: Query<(&mut Transform, &Movement, &ActionState<PlayerAction>), With<Player>>,
) {
    for (mut position, movement_component, action_state) in query.iter_mut() {
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction) {
                if let Some(direction) = input_direction.direction() {
                    let last_position = position.get();
                    let new_position = last_position + direction.coord();
                    if let Some(map) = manager.get_current_map_mut() {
                        if map.try_move_actor(last_position, new_position, movement_component.0) {
                            position.set_value(new_position);
                            info!(
                                "Player moved {:?} from {:?} to {:?}",
                                direction, last_position, new_position
                            );
                        } else {
                            info!("{:?} is blocked!", new_position);
                        }
                    }
                    state.set_next(&mut commands);
                }
            }
        }
    }
}
