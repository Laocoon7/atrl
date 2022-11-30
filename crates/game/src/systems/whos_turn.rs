use crate::prelude::*;

pub fn perform_turns(
    state: Res<TurnState>,
    mut commands: Commands,
    player_q: Query<&Player>,
    mut ai_q: Query<&mut AIComponent>,
    mut turn_manager: ResMut<TurnManager>,
    mut action_queue: ResMut<ActionQueue>,
    mut move_events: EventWriter<WantsToMove>,
    mut waiting_on_entity: Local<Option<(Entity, bool)>>, // (entity, is_player)
) {
    let (entity, action) = if let Some((entity, is_player)) = *waiting_on_entity {
        if is_player {
            if let Some(action_to_perform) = action_queue.get_action() {
                (entity, action_to_perform)
            } else {
                // Waiting on player to decide what to do...
                return;
            }
        } else if let Ok(mut ai_component) = ai_q.get_mut(entity) {
            if let Some(action_to_perform) = std::mem::take(&mut ai_component.preferred_action) {
                (entity, action_to_perform)
            } else {
                info!("Waiting for ai to decide what to do...");
                if *state != TurnState::AIThinking {
                    commands.insert_resource(TurnState::AIThinking);
                }
                return;
            }
        } else {
            error!("Waiting on ai without ai_component!");
            return;
        }
    } else if let Some(entity) = turn_manager.start_entity_turn() {
        commands.entity(entity).insert(MyTurn);

        if player_q.get(entity).is_ok() {
            if let Some(action_to_perform) = action_queue.get_action() {
                (entity, action_to_perform)
            } else {
                *waiting_on_entity = Some((entity, true));
                return;
            }
        } else if let Ok(mut ai_component) = ai_q.get_mut(entity) {
            // Get AI Action
            if let Some(action_to_perform) = std::mem::take(&mut ai_component.preferred_action) {
                (entity, action_to_perform)
            } else {
                *waiting_on_entity = Some((entity, false));
                return;
            }
        } else {
            error!("Unknown entity!");
            return;
        }
    } else {
        error!("No actors in turn manager.");
        return;
    };

    // We have an entity, and an action
    *waiting_on_entity = None;

    let perform_time = action.get_base_time_to_perform();

    commands.entity(entity).remove::<MyTurn>();
    turn_manager.end_entity_turn(entity, perform_time);
}
