use crate::prelude::*;

pub fn move_actors(
    q_map: Query<&GameMap>,
    mut move_events: EventReader<WantsToMove>,
    mut pos_q: Query<(&mut Transform, &WorldPosition, &Movement)>,
) {
    for WantsToMove(entity, destination) in move_events.iter() {
        if let Ok((mut position, world_pos, movement_component)) = pos_q.get_mut(*entity) {
            let map = q_map.iter().find(|m| m.world_position == *world_pos).unwrap();
            if map.can_move_through(*destination, movement_component) {
                println!("Player moved from: {:?} to {:?}", position.get(), destination);
                position.set_value(*destination);
            } else {
                println!("Blocked!");
            }
        }
    }
}
