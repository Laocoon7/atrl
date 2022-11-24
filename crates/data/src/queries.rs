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
