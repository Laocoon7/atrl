use crate::prelude::*;

pub fn perform_action(entity: Entity, action: ActionType, world: &mut World) -> Result<u32, ActionType> {
    match action {
        ActionType::Wait => {
            info!("Waiting");
            Ok(ActionType::Wait.get_base_time_to_perform())
        },
        ActionType::MovementDelta(delta) => {
            let mut position_q = world.query::<&mut Position>();
            position_q.get(world, entity).map_or(Err(ActionType::Wait), |entity_position| {
                Err(ActionType::Movement(*entity_position + delta))
            })
        },
        ActionType::Movement(destination) => match try_move(entity, destination, world) {
            Ok(_) => Ok(action.get_base_time_to_perform()),
            Err(a) => Err(a),
        },
        ActionType::Attack(position) => match try_attack(entity, position, world) {
            Ok(_) => Ok(action.get_base_time_to_perform()),
            Err(a) => Err(a),
        },
    }
}
