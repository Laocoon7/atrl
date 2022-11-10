use crate::game::prelude::*;

pub fn player_input(
    mut query: Query<(&mut LocalPosition, &ActionState<PlayerAction>), With<Player>>,
) {
    let (mut player_pos, action_state) = query.single_mut();

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(input_direction) {
            if let Some(direction) = input_direction.direction() {
                let destination = player_pos.position + direction.coord();
                player_pos.position = destination;
            }
        }
    }
}
