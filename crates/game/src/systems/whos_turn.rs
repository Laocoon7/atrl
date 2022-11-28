use crate::prelude::*;

pub fn whos_turn(
    mut commands: Commands,
    // state: Res<TurnState>,
    player_q: Query<&Player>,
    my_turn_q: Query<&MyTurn>,
    mut turn_manager: ResMut<TurnManager>,
) {
    if !my_turn_q.is_empty() {
        return;
    }

    let mut player_turn = false;
    if let Some(entity) = turn_manager.start_entity_turn() {
        println!("Turn for: {:?}", entity);

        commands.entity(entity).insert(MyTurn);

        if player_q.get(entity).is_ok() {
            player_turn = true;
        }
    }

    if player_turn {
        // give player priority to take their turn
        // state.set_next(&mut commands)
        println!("Player's whos turn!");
        commands.insert_resource(TurnState::AwaitingInput);
    }
}
