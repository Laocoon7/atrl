use crate::prelude::*;

pub fn player_input(
    state: Res<TurnState>,
    mut commands: Commands,
    mut query: Query<
        (&mut Transform, &WorldPosition, &Movement, &ActionState<PlayerAction>),
        With<Player>,
    >,
    q_map: Query<&GameMap>,
) {
    for (mut position, world_position, movement_component, action_state) in query.iter_mut() {
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction) {
                if let Some(direction) = input_direction.direction() {
                    let mut position_vec = position.get();
                    position_vec += direction.coord().as_vec2();

                    for map in q_map.iter() {
                        if map.world_position.position == world_position.position {
                            if map.can_move_through(
                                (position_vec.x, position_vec.y),
                                movement_component,
                            ) {
                                position.set_value(position_vec);
                                println!(
                                    "Player moved to {} {:?} {:?}",
                                    position.translation,
                                    direction,
                                    direction.coord().as_vec2()
                                );
                            } else {
                                println!("Blocked!");
                            }
                            break;
                        }
                    }

                    state.set_next(&mut commands);
                }
            }
        }
    }
}
