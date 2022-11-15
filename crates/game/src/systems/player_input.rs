use crate::prelude::*;

pub fn player_input(
    state: Res<TurnState>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &ActionState<PlayerAction>), With<Player>>,
) {
    let (mut position, action_state) = query.single_mut();

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.just_pressed(input_direction) {
            if let Some(direction) = input_direction.direction() {
                let mut position_vec = position.get();
                position_vec += direction.coord().as_vec2();
                position.set_value(position_vec);

                println!("Player moved to {:?} {:?}", direction, direction.coord().as_vec2());

                state.set_next(&mut commands);
            }
        }
    }
}
