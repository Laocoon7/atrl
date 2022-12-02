use crate::prelude::*;

// FIX: PERFORMANCE??
pub fn entity_in_fov(
    map: &Map,
    fov: &FieldOfView,
    vision: &Vision,
    current_pos: UVec2,
    destination_pos: UVec2,
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

// We assume the attack range is sqrt(2)
pub fn in_attack_range(attacker_pos: UVec2, victim_pos: UVec2) -> bool {
    let distance = DistanceAlg::Pythagoras.distance2d(attacker_pos, victim_pos);
    distance <= DIAGONAL_COST
}
