use crate::prelude::*;

pub fn entity_in_fov(
    map: &Map,
    fov: &FieldOfView,
    vision: &Vision,
    current_pos: impl Point2d,
    destination_pos: impl Point2d,
) -> bool {
    // // If the player is within the FOV range of the AI, check line of sight
    let line_length = grid_shapes::Line::new(current_pos, destination_pos).get_count();
    if line_length < fov.0 as usize {
        let mut visibility_map = VisibilityMap::new(map.size);
        let angle = (destination_pos.angle_to(current_pos) - 180.0).abs();
        Fov::ShadowcastDirection(CardinalDirection::from(angle as i32)).compute(
            current_pos,
            vision.0,
            fov.0,
            map,
            &mut visibility_map,
        );

        visibility_map.get_visible(destination_pos.as_ivec2())
    } else {
        false
    }
}

pub fn update_target_visual(
    commands: &mut Commands,
    tilesets: &Tilesets,
    target_q: &mut Query<&mut TargetVisualizer>,
    ai_path: &Vec<IVec2>,
    actor: &Entity,
    next_pt: &IVec2,
    color: Color,
) {
    if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
        if !ai_path.is_empty() {
            target_visualizer.update(commands, tilesets, *next_pt, ai_path[0], color);
        } else {
            target_visualizer.update(commands, tilesets, *next_pt, *next_pt, color);
        }
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
