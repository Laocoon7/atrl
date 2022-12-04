use crate::prelude::*;

pub fn try_attack(entity: Entity, pos: Position, world: &mut World) -> Result<(), ActionType> {
    // TODO: Rewrite

    //    world.resource_scope(|world, mut map_manager: MapManager| {
    //
    //
    //
    //        map_manager.get_current_map().map_or_else(
    //            || {
    //                info!("Couldn't find entities health component: ");
    //                Err(ActionType::Wait)
    //            },
    //            |map| {
    //                map.get_actor(pos).map_or(Err(ActionType::Wait), |victim| {
    //                    let mut health_q = world.query::<(&mut Health, &Name)>();
    //
    //                    health_q.get_mut(world, victim).map_or_else(
    //                        |err| {
    //                            info!("Couldn't find entities health component: {}", err);
    //                            Err(ActionType::Wait)
    //                        },
    //                        |(mut actor_health, name)| {
    //                            let before = format!("{}/{}", actor_health.current_hp,
    // actor_health.max_hp);                            actor_health.current_hp -= 1;
    //                            let after = format!("{}/{}", actor_health.current_hp,
    // actor_health.max_hp);
    //
    //                            println!(
    //                                "{} is attacking {:?} before: ({}) after: ({})",
    //                                name, entity, before, after
    //                            );
    //
    //                            Ok(())
    //                        },
    //                    )
    //                })
    //            },
    //        )
    //    })
    Ok(())
}
