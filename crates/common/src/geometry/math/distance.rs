use std::ops::Sub;

use crate::prelude::*;

const CARDINAL_COST: f32 = 1.0;
pub const DIAGONAL_COST: f32 = 1.4142135623730950488016887242096980785696718753769480731766797379;

/// Enumeration of available 2D Distance algorithms
#[allow(clippy::module_name_repetitions)]
pub enum DistanceAlg {
    /// Use the Pythagoras algorithm for determining distance - sqrt(A^2 + B^2)
    Pythagoras,
    /// Us the Pythagoras algorithm for distance, but omitting the square-root for a faster but
    /// squared result.
    PythagorasSquared,
    /// Use Manhattan distance (distance up plus distance along)
    Manhattan,
    /// Use Chebyshev distance (like Manhattan, but adds one to each entry)
    Chebyshev,
    /// Use a diagonal distance, the max of the x and y distances
    Diagonal,
    /// Use a diagonal distance, the max of the x and y distances
    DiagonalWithCosts(f32, f32,),
}

impl DistanceAlg {
    /// Provides a 2D distance between points, using the specified algorithm.

    /// Provides a 2D distance between points, using the specified algorithm.
    pub fn distance2d(self, start: impl Point2d, end: impl Point2d,) -> f32 {
        match self {
            Self::Pythagoras => distance2d_pythagoras(start, end,),
            Self::PythagorasSquared => distance2d_pythagoras_squared(start, end,),
            Self::Manhattan => distance2d_manhattan(start, end,),
            Self::Chebyshev => distance2d_chebyshev(start, end,),
            Self::Diagonal => distance2d_diagonal(start, end,),
            Self::DiagonalWithCosts(d1, d2,) => distance2d_diagonal_with_costs(start, end, d1, d2,),
        }
    }
}

/// Calculates a Pythagoras distance between two points.
fn distance2d_pythagoras(start: impl Point2d, end: impl Point2d,) -> f32 {
    let distance_squared = distance2d_pythagoras_squared(start, end,);
    f32::sqrt(distance_squared,)
}

/// Calculates a Pythagoras distance between two points, and skips the square root for speed.
fn distance2d_pythagoras_squared(start: impl Point2d, end: impl Point2d,) -> f32 {
    let start = start.as_vec2();
    let end = end.as_vec2();

    let distance = (start.max(end,) - start.min(end,)).powf(2.0,);
    distance.x + distance.y
}

/// Calculates a Manhattan distance between two points
fn distance2d_manhattan(start: impl Point2d, end: impl Point2d,) -> f32 {
    let start = start.as_vec2();
    let end = end.as_vec2();
    let distance = start.max(end,) - start.min(end,);
    distance.x + distance.y
}

/// Calculates a Chebyshev distance between two points
/// See: [GameProgramming/Heuristics](http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html)
fn distance2d_chebyshev(start: impl Point2d, end: impl Point2d,) -> f32 {
    // distance2d_diagonal_with_costs(start, end, 1.0, 1.0)
    let start = start.as_vec2();
    let end = end.as_vec2();

    start.sub(end,).abs().max_element()
}

// Calculates a diagonal distance
fn distance2d_diagonal(start: impl Point2d, end: impl Point2d,) -> f32 {
    distance2d_diagonal_with_costs(start, end, CARDINAL_COST, DIAGONAL_COST,)
}

// Calculates a diagonal distance
fn distance2d_diagonal_with_costs(
    start: impl Point2d, end: impl Point2d, d1: f32, d2: f32,
) -> f32 {
    let start = start.as_vec2();
    let end = end.as_vec2();

    let distance = start.sub(end,).abs();
    d1.mul_add(distance.max_element(), (d2 - d1) * distance.min_element(),)
}
