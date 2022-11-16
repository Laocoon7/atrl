use crate::prelude::*;

pub fn player_input(
    state: Res<TurnState>,
    mut commands: Commands,
    mut move_writer: EventWriter<WantsToMove>,
    query: Query<(Entity, &Transform, &ActionState<PlayerAction>), With<Player>>,
) {
    for (player, position, action_state) in query.iter() {
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction) {
                if let Some(direction) = input_direction.direction() {
                    let mut position_vec = position.get();
                    position_vec += direction.coord().as_vec2();
                    move_writer.send(WantsToMove(player, position_vec));
                    state.set_next(&mut commands);
                }
            }
        }
    }
}
