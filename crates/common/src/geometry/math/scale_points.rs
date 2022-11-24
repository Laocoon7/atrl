use crate::prelude::*;

/// Scale `points` (move them towards or away) around the `center` by `factor`
///
/// The resulting points distance will be points[x].distance(center) * factor but at the same
/// angle
pub fn scale_points(center: impl Point2d, points: &[IVec2], factor: f32) -> Vec<IVec2> {
    let mut output = vec![];
    for point in points {
        let angle = center.angle_to(*point);
        let dist = DistanceAlg::Pythagoras.distance2d(center, *point) as f32 * factor;
        output.push(IVec2::from_angle(center, dist.max(0.0), angle));
    }
    output
}
