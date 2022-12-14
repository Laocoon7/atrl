use crate::prelude::*;

// FIX: PERFORMANCE??
pub fn entity_in_fov<Range: Into<u32>>(
    map_manager: &mut impl FovProvider,
    q_blocks_vision: &Query<&'static BlocksVision>,
    range: Range,
    vision: &Vision,
    current_pos: Position,
    destination_pos: Position,
) -> bool {
    // // If the player is within the FOV range of the AI, check line of sight
    let distance = current_pos.distance(destination_pos);
    let range = range.into();
    if distance <= range {
        let mut visibility_map = VisibilityMap::new();
        let octant = current_pos.octant_to(destination_pos);
        let direction = CardinalDirection::from_octant(octant);
        Fov::ShadowcastDirection(direction).compute(
            current_pos,
            vision.0,
            range,
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
