use crate::prelude::*;

pub fn try_attack(entity: Entity, pos: Position, world: &mut World) -> Result<(), ActionType> {
    let mut health_q = world.query::<&mut Health>();
    health_q.get_mut(world, entity).map_or_else(
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
