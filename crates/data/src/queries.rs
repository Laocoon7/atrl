use crate::prelude::*;

pub fn entity_in_fov(
    map: &Map,
    fov: &FieldOfView,
    vision: &Vision,
    self_pos: impl Point2d,
    other_pos: impl Point2d,
) -> bool {
    // If the player is within the FOV range of the AI, check line of sight
    if DistanceAlg::Pythagoras.distance2d(self_pos, other_pos) < fov.0 as f32 {
        let line = Line::new(self_pos, other_pos);
        line.iter().all(|point| !map.is_opaque(point, vision.0))
    } else {
        false
    }
}

pub fn end_turn_requeue(
    commands: &mut Commands,
    turn_manager: &mut TurnManager,
    entity: Entity,
    _ticks_until_turn: u32,
) {
    println!("End turn: {:#?}", entity);

    // FIX: What should this be?
    let time_spent = TURN_TIME;

    turn_manager.end_entity_turn(entity, time_spent);
    commands.entity(entity).remove::<MyTurn>();
    // commands.entity(entity).remove::<MyTurn>().insert(WaitingForTurn {
    //     next_turn_tick: todo!(),
    // });
}
