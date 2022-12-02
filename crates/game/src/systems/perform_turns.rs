use crate::prelude::*;

pub fn perform_turns(
    q_player: Query<&Player>,
    mut q_ai: Query<(&mut AIComponent, &Name)>,

    q_movement: Query<&Movement>,
    mut q_position: Query<&mut Position>,

    mut map_manager: ResMut<MapManager>,
    mut turn_manager: ResMut<TurnManager>,
    mut action_queue: ResMut<ActionQueue>,
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
            } else if let Ok((mut ai_component, name)) = q_ai.get_mut(entity) {
                info!("Starting turn for {}", name);

                if let Some(a) = std::mem::take(&mut ai_component.preferred_action) {
                    info!("{} is performing {:?}", name, a);
                    a
                } else {
                    info!("{} has no preferred action!", name);
                    turn_manager.end_entity_turn(entity, 0);
                    return;
                    // ActionType::Wait
                }
            } else {
                info!("AI does not have an AI Component.");
                // don't add the entity back to the queue...
                // just go to the next one and try to recover
                continue;
            };

            loop {
                match perform_action(
                    entity,
                    action,
                    &mut map_manager,
                    &mut q_position,
                    &q_movement,
                ) {
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

fn perform_action(
    entity: Entity,
    action: ActionType,
    map_manager: &mut ResMut<MapManager>,
    q_position: &mut Query<&mut Position>,
    q_movement: &Query<&Movement>,
) -> Result<u32, ActionType> {
    match action {
        ActionType::Wait => {
            info!("Waiting");
            Ok(ActionType::Wait.get_base_time_to_perform())
        },
        ActionType::Movement(destination) => {
            match try_move(entity, map_manager, q_position, q_movement, destination) {
                Ok(_) => Ok(action.get_base_time_to_perform()),
                Err(a) => Err(a),
            }
        },
        ActionType::MovementDelta(delta) => {
            if let Ok(entity_position) = q_position.get(entity) {
                Err(ActionType::Movement(*entity_position + delta))
            } else {
                Err(ActionType::Wait)
            }
        },
    }
}

fn try_move(
    entity: Entity,
    map_manager: &mut ResMut<MapManager>,
    q_position: &mut Query<&mut Position>,
    q_movement: &Query<&Movement>,
    destination: Position,
) -> Result<(), ActionType> {
    if let Ok(mut from_position) = q_position.get_mut(entity) {
        if let Ok(movement_component) = q_movement.get(entity) {
            if let Some(map) = map_manager.get_current_map_mut() {
                // try to generate a path.
                if let Some(mut path) = PathFinder::Astar.compute(
                    from_position.xy(),
                    destination.xy(),
                    movement_component.0,
                    true,
                    map,
                ) {
                    // move one space on that path.
                    if let Some(destination) = path.pop() {
                        // TODO: check if is closed door
                        // and return Err(ActionType::OpenDoor(destination))
                        if map.try_move_actor(from_position.xy(), destination, movement_component.0) {
                            from_position.set_xy(destination.as_uvec2());
                            Ok(())
                        } else {
                            info!("{:?} is blocked!", destination);
                            Err(ActionType::Wait)
                        }
                    } else {
                        info!("Couldn't find a long enough path to {:?}", destination);
                        Err(ActionType::Wait)
                    }
                } else {
                    info!("Couldn't find a path to {:?}", destination);
                    Err(ActionType::Wait)
                }
            } else {
                info!("Couldn't find the map.");
                Err(ActionType::Wait)
            }
        } else {
            info!("Couldn't find a movement component.");
            Err(ActionType::Wait)
        }
    } else {
        info!("Couldn't find entities position components.");
        Err(ActionType::Wait)
    }
}
