use crate::prelude::*;

#[derive(Resource)]
pub struct CachedTurnSystemState(
    pub  SystemState<(
        ResMut<'static, MapManager>,
        ResMut<'static, TurnManager>,
        ResMut<'static, ActionQueue>,
        Query<'static, 'static, &'static Player>,       // player query
        Query<'static, 'static, &'static Movement>,     // movement query
        Query<'static, 'static, &'static mut Position>, // position query
        Query<'static, 'static, (&'static mut AIComponent, &'static Name)>, // ai query
        Query<'static, 'static, &'static mut Health>,   // health query
    )>,
);

impl FromWorld for CachedTurnSystemState {
    fn from_world(world: &mut World) -> Self { Self(SystemState::new(world)) }
}

pub fn perform_turns(world: &mut World) {
    world.resource_scope(|world, mut cached_state: Mut<CachedTurnSystemState>| {
        let (
            mut map_manager,
            mut turn_manager,
            mut action_queue,
            q_player,
            q_movement,
            mut q_position,
            mut q_ai,
            mut health_q,
        ) = cached_state.0.get_mut(world);

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
                    }
                } else {
                    info!("AI does not have an AI Component.");
                    // don't add the entity back to the queue...
                    // just go to the next one and try to recover
                    return;
                };

                loop {
                    match perform_action(
                        entity,
                        action,
                        &mut map_manager,
                        &mut q_position,
                        &q_movement,
                        &mut health_q,
                    ) {
                        Ok(time_spent) => {
                            turn_manager.end_entity_turn(entity, time_spent);
                            break;
                        },
                        Err(a) => action = a,
                    }
                }

                if is_player {
                    for (mut ai_component, _name) in q_ai.iter_mut() {
                        ai_component.preferred_action = None;
                    }
                    return;
                }
            } else {
                error!("No entities waiting for a turn!");
                return;
            }
        }
    });
}
