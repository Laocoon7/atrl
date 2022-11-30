use crate::prelude::*;

pub fn perform_turns(
    mut commands: Commands,
    state: Res<TurnState>,

    q_player: Query<&Player>,
    mut q_ai: Query<&mut AIComponent>,

    mut q_position: Query<(&mut WorldPosition, &mut LocalPosition)>,
    
    mut map_manager: ResMut<MapManager>,
    mut turn_manager: ResMut<TurnManager>,
    mut action_queue: ResMut<ActionQueue>,

    mut move_events: EventWriter<WantsToMove>,
) {
    loop {
        // Select next entity
        if let Some(entity) = turn_manager.start_entity_turn() {
            let is_player = q_player.get(entity).is_ok();

            let mut action = if is_player {
                if let Some(a) = action_queue.get_action() {
                    a
                } else {
                    turn_manager.end_entity_turn(entity, 0);
                    return;
                }
            } else {
                if let Ok(mut ai_component) = q_ai.get_mut(entity) {
                    if let Some(a) = std::mem::take(&mut ai_component.preferred_action) {
                        a
                    } else {
                        turn_manager.end_entity_turn(entity, 0);
                        return;
                    }
                } else {
                    error!("AI does not have an AI Component.");
                    // don't add the entity back to the queue...
                    // just go to the next one and try to recover
                    continue;
                }
            };

            loop {
                match perform_action(entity, action, &mut map_manager, &mut q_position) {
                    Ok(time_spent) => {
                        turn_manager.end_entity_turn(entity, time_spent);
                        break;
                    },
                    Err(a) => action = a,
                }
            }

        } else {
            error!("No entities waiting for a turn!");
            return;
        }
    }
}

fn perform_action(entity: Entity, action: ActionType, map_manager: &mut ResMut<MapManager>, q_position: &mut Query<(&mut WorldPosition, &mut LocalPosition)>) -> Result<u32, ActionType> {
    match action {
        ActionType::Wait => Ok(ActionType::Wait.get_base_time_to_perform()),
        ActionType::Movement(destination) => {
            if try_move(entity, map_manager, q_position, destination.0, destination.1).is_ok() {
                return Ok(action.get_base_time_to_perform())
            } else {
                return Err(ActionType::Wait)
            }
        }
        ActionType::MovementDelta(delta) => {
            if let Ok((world_position, local_position)) = q_position.get(entity) {
                let mut world_position = world_position.0;
                let mut local_position = local_position.0.as_ivec2() + delta;

                if local_position.x < 0 {
                    local_position.x += GRID_WIDTH as i32;
                    world_position.x -= 1;
                } else if local_position.x >= GRID_WIDTH as i32 {
                    local_position.x -= GRID_WIDTH as i32;
                    world_position.x += 1;
                }

                if local_position.y < 0 {
                    local_position.y += GRID_HEIGHT as i32;
                    world_position.y -= 1;
                } else if local_position.y >= GRID_HEIGHT as i32 {
                    local_position.y -= GRID_HEIGHT as i32;
                    world_position.y += 1;
                }
                return Err(ActionType::Movement((world_position, local_position.as_uvec2())))
            } else {
                return Err(ActionType::Wait)
            }
            
        }
    }
}

fn try_move(entity: Entity, map_manager: &mut ResMut<MapManager>, q_position: &mut Query<(&mut WorldPosition, &mut LocalPosition)>, _world_destination: IVec3, local_destination: UVec2) -> Result<(), ()> {
    
    // try to generate a path.

    // move one space on that path.

    // return ok if successful

    Err(())
}