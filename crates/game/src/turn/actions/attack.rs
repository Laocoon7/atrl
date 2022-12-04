use crate::prelude::*;

pub fn try_attack(
    entity: Entity,
    pos: Position,
    health_q: &mut Query<&mut Health>,
) -> Result<(), ActionType> {
    health_q.get_mut(entity).map_or_else(
        |err| {
            info!("Couldn't find entities health component: {}", err);
            Err(ActionType::Wait)
        },
        |mut actor_health| {
            actor_health.current_hp -= 1;
            Ok(())
        },
    )
}
