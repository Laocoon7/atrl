use crate::prelude::*;

pub fn movement(
    mut map_manager: ResMut<MapManager>,
    mut move_events: ResMut<Events<WantsToMove>>,
    mut spatial_q: Query<(&mut Transform, &Movement), With<WorldPosition>>,
) {
    for WantsToMove(entity, destination) in move_events.drain() {
        let Some(map) = map_manager.get_current_map_mut() else {
            error!("No map found");
            return
        };
        let Ok((mut position, Movement(movement_type))) = spatial_q.get_mut(entity) else {
            error!("Entity does not have spatial components");
            return
        };

        let last_position = position.get();
        let new_position = destination;

        if map.try_move_actor(last_position, new_position, *movement_type) {
            position.set_value(new_position);
            info!("Actor moved from {:?} to {:?}", last_position, new_position);
        } else {
            info!(
                "From:{:?} To:{:?} is blocked! {:?}",
                last_position, new_position, *movement_type
            );
        }
    }
}
