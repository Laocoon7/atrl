use crate::prelude::*;

pub fn player_input(
    state: Res<TurnState>,
    mut commands: Commands,
    map_q: Query<&Map>,
    mut query: Query<
        (&mut Transform, &WorldPosition, &Movement, &ActionState<PlayerAction>),
        With<Player>,
    >,
) {
    for (mut position, world_position, movement_component, action_state) in query.iter_mut() {
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction) {
                if let Some(direction) = input_direction.direction() {
                    let mut position_vec = position.get();
                    position_vec += direction.coord().as_vec2();

                    if let Some(map) =
                        map_q.iter().find(|map| map.world_position == *world_position)
                    {
                        if map.world_position == *world_position {
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
                        }
                    }

                    state.set_next(&mut commands);
                }
            }
        }
    }
}
