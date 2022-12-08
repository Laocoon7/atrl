use crate::prelude::*;

pub const MOVE_TIME: u32 = SECONDS * 2;

#[derive(Debug, Clone)]
pub struct MovementAction(pub Position);

impl AtrlAction for MovementAction {
    fn get_base_time_to_perform(&self) -> u32 { MOVE_TIME }

    fn perform(&mut self, world: &mut World, entity: Entity) -> Result<u32, BoxedAction> {
        let destination = self.0;
        match try_move(world, entity, destination) {
            Ok(_) => {
                info!("Movement({})", destination);
                Ok(self.get_base_time_to_perform())
            },
            Err(a) => Err(a),
        }
    }

    fn get_target_position(&self) -> Option<Position> { Some(self.0) }
}

#[derive(Debug, Clone)]
pub struct MovementDeltaAction(pub IVec2);

impl AtrlAction for MovementDeltaAction {
    fn get_base_time_to_perform(&self) -> u32 { MOVE_TIME }

    fn perform(&mut self, world: &mut World, entity: Entity) -> Result<u32, BoxedAction> {
        let mut position_q = world.query::<&mut Position>();
        position_q.get(world, entity).map_or(Err(WaitAction.boxed()), |entity_position| {
            let delta = self.0;
            info!("MovementDelta({:?}) from {}", delta, entity_position);
            Err(MovementAction(*entity_position + delta).boxed())
        })
    }
}

pub fn try_move(world: &mut World, entity: Entity, destination: Position) -> Result<(), BoxedAction> {
    let mut system_state: SystemState<(
        MapManager,
        Query<(&mut Position, &Movement)>,
        Query<&BlocksMovement>,
    )> = SystemState::new(world);
    let (mut map_manager, mut spatial_q, q_blocks_movement) = system_state.get_mut(world);

    spatial_q.get_mut(entity).map_or_else(
        |err| {
            info!("Couldn't find entities position components: {}", err);
            Err(WaitAction.boxed())
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
                        Err(WaitAction.boxed())
                    },
                    |mut path| {
                        path.pop().map_or_else(
                            || {
                                info!("Couldn't find a long enough path to {:?}", destination);
                                Err(WaitAction.boxed())
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
                                    Err(WaitAction.boxed())
                                }
                            },
                        )
                    },
                )
        },
    )
}
