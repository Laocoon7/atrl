use crate::prelude::*;

pub fn try_move(
    entity: Entity,
    destination: Position,
    world: &mut World,
    // map_manager: &mut ResMut<MapManager>,
    // q_position: &mut Query<&mut Position>,
    // q_movement: &Query<&Movement>,
) -> Result<(), ActionType> {
    world.resource_scope(|world, mut map_manager: Mut<MapManager>| {
        let mut spatial_q = world.query::<(&mut Position, &Movement)>();

        spatial_q.get_mut(world, entity).map_or_else(
            |err| {
                info!("Couldn't find entities position components: {}", err);
                Err(ActionType::Wait)
            },
            |(mut from_position, movement_component)| {
                map_manager.get_current_map_mut().map_or_else(
                    || {
                        info!("Couldn't find the map.");
                        Err(ActionType::Wait)
                    },
                    |map| {
                        PathFinder::Astar
                            .compute(
                                from_position.gridpoint(),
                                destination.gridpoint(),
                                movement_component.0,
                                true,
                                map,
                            )
                            .map_or_else(
                                || {
                                    info!("Couldn't find a path to {:?}", destination);
                                    Err(ActionType::Wait)
                                },
                                |mut path| {
                                    path.pop().map_or_else(
                                        || {
                                            info!("Couldn't find a long enough path to {:?}", destination);
                                            Err(ActionType::Wait)
                                        },
                                        |destination| {
                                            if map.try_move_actor(
                                                from_position.gridpoint(),
                                                destination.as_uvec2(),
                                                movement_component.0,
                                            ) {
                                                from_position.set_xy(destination.as_uvec2());
                                                Ok(())
                                            } else {
                                                info!("{:?} is blocked!", destination);
                                                Err(ActionType::Wait)
                                            }
                                        },
                                    )
                                },
                            )
                    },
                )
            },
        )
    })
}
