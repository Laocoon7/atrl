use crate::prelude::*;

/// Rotate `points` around the `center` by `degrees`
///
/// The resulting points at the same distance but at +degrees angle
pub fn rotate_points(center: impl Point2d, points: &[IVec2], degrees: f32,) -> Vec<IVec2,> {
    let mut output = vec![];
    for point in points {
        let starting_angle = center.angle_to(*point,);
        let dist = DistanceAlg::Pythagoras.distance2d(center, *point,);
        output.push(IVec2::from_angle(center, dist, starting_angle + degrees,),);
    }
    output
}
