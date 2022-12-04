use crate::prelude::*;
/// An implementation of [Bresenham's circle algorithm].
/// [Bresenham's circle algorithm]: http://members.chello.at/~easyfilter/bresenham.html
/// Derived from the line_drawing crate, but specialized to use BTerm's types.
#[derive(Debug, Clone)]
pub struct BresenhamCircleIter {
    x: i32,
    y: i32,
    error: i32,
    radius: i32,
    quadrant: u8,
    center: IVec2,
}
impl BresenhamCircleIter {
    /// Creates a new circle, using the Bresenham Circle algorithm.
    ///
    /// # Arguments
    ///
    /// * `center` - the center of the circle.
    /// * `radius` - the radius of the desired circle.
    #[inline]
    #[allow(dead_code)]
    pub fn new(center: impl GridPoint, radius: i32) -> Self {
        Self {
            y: 0,
            radius,
            x: -radius,
            quadrant: 1,
            error: 2 - 2 * radius,
            center: center.as_ivec2(),
        }
    }
}
impl Iterator for BresenhamCircleIter {
    type Item = IVec2;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < 0 {
            let point = match self.quadrant {
                1 => (self.center.x - self.x, self.center.y + self.y),
                2 => (self.center.x - self.y, self.center.y - self.x),
                3 => (self.center.x + self.x, self.center.y - self.y),
                4 => (self.center.x + self.y, self.center.y + self.x),
                _ => unreachable!(),
            }
            .as_ivec2();
            // Update the variables after each set of quadrants
            if self.quadrant == 4 {
                self.radius = self.error;
                if self.radius <= self.y {
                    self.y += 1;
                    self.error += self.y * 2 + 1;
                }
                if self.radius > self.x || self.error > self.y {
                    self.x += 1;
                    self.error += self.x * 2 + 1;
                }
            }
            self.quadrant = self.quadrant % 4 + 1;
            Some(point)
        } else {
            None
        }
    }
}
/// A version of the Bresenham circle that does not make diagonal jumps
pub struct BresenhamCircleNoDiagIter {
    x: i32,
    y: i32,
    error: i32,
    quadrant: u8,
    center: IVec2,
}
impl BresenhamCircleNoDiagIter {
    /// Creates a Bresenham Circle without allowing diagonal gaps.
    ///
    /// # Arguments
    ///
    /// * `center` - the center of the circle
    /// * `radius` - the radius of the circle
    #[inline]
    #[allow(dead_code)]
    pub fn new(center: impl GridPoint, radius: i32) -> Self {
        Self {
            center: center.as_ivec2(),
            x: -radius,
            y: 0,
            error: 0,
            quadrant: 1,
        }
    }
}
impl Iterator for BresenhamCircleNoDiagIter {
    type Item = IVec2;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < 0 {
            let point = match self.quadrant {
                1 => (self.center.x - self.x, self.center.y + self.y),
                2 => (self.center.x - self.y, self.center.y - self.x),
                3 => (self.center.x + self.x, self.center.y - self.y),
                4 => (self.center.x + self.y, self.center.y + self.x),
                _ => unreachable!(),
            }
            .as_ivec2();
            // Update the variables after each set of quadrants.
            if self.quadrant == 4 {
                // This version moves in x or in y - not both - depending on the error.
                if (self.error + 2 * self.x + 1).abs() <= (self.error + 2 * self.y + 1).abs() {
                    self.error += self.x * 2 + 1;
                    self.x += 1;
                } else {
                    self.error += self.y * 2 + 1;
                    self.y += 1;
                }
            }
            self.quadrant = self.quadrant % 4 + 1;
            Some(point)
        } else {
            None
        }
    }
}
