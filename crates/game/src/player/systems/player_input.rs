use crate::prelude::*;

pub fn player_input(
    state: Res<TurnState>,
    mut commands: Commands,
    manager: Res<MapManager>,
    mut query: Query<(&mut Transform, &Movement, &ActionState<PlayerAction>), With<Player>>,
) {
    for (mut position, movement_component, action_state) in query.iter_mut() {
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction) {
                if let Some(direction) = input_direction.direction() {
                    let mut position_vec = position.get();
                    position_vec += direction.coord();

                    if let Some(map) = manager.get_current_map() {
                        if map.can_move_through(position_vec, movement_component) {
                            position.set_value(position_vec);
                            info!(
                                "Player moved to {} {:?} {:?}",
                                position.translation,
                                direction,
                                direction.coord().as_vec2()
                            );
                        } else {
                            info!("Blocked!");
                        }
                    }

                    state.set_next(&mut commands);
                }
            }
        }
    }
}
