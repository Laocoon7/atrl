use crate::prelude::*;

pub fn perform_action(
    entity: Entity,
    action: ActionType,
    map_manager: &mut ResMut<MapManager>,
    q_position: &mut Query<&mut Position>,
    q_movement: &Query<&Movement>,
    health_q: &mut Query<&mut Health>,
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
            q_position.get(entity).map_or(Err(ActionType::Wait), |entity_position| {
                Err(ActionType::Movement(*entity_position + delta))
            })
        },
        ActionType::Attack(position) => match try_attack(entity, position, health_q) {
            Ok(_) => Ok(action.get_base_time_to_perform()),
            Err(a) => Err(a),
        },
    }
}
