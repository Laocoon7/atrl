use crate::prelude::*;

pub fn try_move(
    entity: Entity,
    destination: Position,
    world: &mut World,
    // map_manager: &mut ResMut<MapManager>,
    // q_position: &mut Query<&mut Position>,
    // q_movement: &Query<&Movement>,
) -> Result<(), ActionType> {
    let mut system_state: SystemState<(
        MapManager,
        Query<(&mut Position, &Movement)>,
        Query<&BlocksMovement>,
    )> = SystemState::new(world);
    let (mut map_manager, mut spatial_q, q_blocks_movement) = system_state.get_mut(world);

    spatial_q.get_mut(entity).map_or_else(
        |err| {
            info!("Couldn't find entities position components: {}", err);
            Err(ActionType::Wait)
        },
        // OPTIMIZE: ActionType: AiPath?? so we only have to calculate this for the AI? it's not terribly
        // expensive having the player do it, but eh??
        |(mut from_position, movement_component)| {
            PathFinder::Astar
                .compute(
                    *from_position,
                    destination,
                    movement_component.0,
                    true,
                    &mut map_manager,
                    &q_blocks_movement,
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
                                if map_manager.move_actor(
                                    entity,
                                    *from_position,
                                    destination,
                                    movement_component.0,
                                    &q_blocks_movement,
                                ) {
                                    *from_position = destination;
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
}
