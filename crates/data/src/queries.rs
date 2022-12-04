use crate::prelude::*;

// FIX: PERFORMANCE??
pub fn entity_in_fov<'w, 's>(
    map_manager: &mut impl FovProvider,
    q_blocks_vision: &Query<'w, 's, &'static BlocksVision>,
    fov: &FieldOfView,
    vision: &Vision,
    current_pos: Position,
    destination_pos: Position,
) -> bool {
    // // If the player is within the FOV range of the AI, check line of sight
    let distance = current_pos.distance(destination_pos);
    if distance < fov.0 as u32 {
        let mut visibility_map = VisibilityMap::new();
        let angle = (destination_pos.gridpoint().angle_to(current_pos.gridpoint()) - 180.0).abs();
        Fov::ShadowcastDirection(CardinalDirection::from(angle as i32)).compute(
            current_pos,
            vision.0,
            fov.0,
            map_manager,
            q_blocks_vision,
            &mut visibility_map,
        );

        visibility_map.get_visible(destination_pos)
    } else {
        false
    }
}

// We assume the attack range is sqrt(2)
pub fn in_attack_range(attacker_pos: Position, victim_pos: Position) -> bool {
    let distance = attacker_pos.distance(victim_pos);
    distance <= 1
}
