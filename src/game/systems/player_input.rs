use crate::game::prelude::internal::*;
use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    state: Res<CurrentGameState>,
    mut query: Query<(&mut LocalPosition, &ActionState<PlayerAction>), With<Player>>,
) {
    let (mut player_pos, action_state) = query.single_mut();

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.just_pressed(input_direction) {
            if let Some(direction) = input_direction.direction() {
                println!("Player moved {:?}", direction);
                let destination = player_pos.position + direction.coord();
                player_pos.position = destination;

                state.set_next(&mut commands);
            }
        }
    }
}
