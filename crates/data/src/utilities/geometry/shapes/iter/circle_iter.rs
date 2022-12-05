use crate::prelude::*;
/// An implementation of [Bresenham's circle algorithm].
/// [Bresenham's circle algorithm]: http://members.chello.at/~easyfilter/bresenham.html
/// Derived from the line_drawing crate, but specialized to use BTerm's types.
#[derive(Debug, Clone)]
pub struct BresenhamCircleIter {
    x: i32,
    y: i32,
    d: i32,
    radius: u32,
    center: Position,
}

impl BresenhamCircleIter {
    /// Creates a new circle, using the Bresenham Circle algorithm.
    ///
    /// # Arguments
    ///
    /// * `center` - the center of the circle.
    /// * `radius` - the radius of the desired circle.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn new(center: Position, radius: u32) -> Self {
        let mut x = 0;
        let mut y = radius as i32;
        let mut d = (5 - (radius as i32 * 4)) / 4;

        Self {
            y,
            x,
            d,
            radius,
            center,
        }
    }
}

impl Iterator for BresenhamCircleIter {
    type Item = Position;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let start = self.center + IVec2::new(self.x, self.y);
        let end = self.center + IVec2::new(self.x, -self.y);
        None
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod line {
        use line_drawing::BresenhamCircle;

        use crate::prelude::*;

        #[test]
        fn radius_4() {
            // let mut p1 = Position::ZERO;
            // p1.set_xy(UVec2::new(5, 5));
            // let mut canvas = Canvas::new([11, 11]);
            // let circle = BresenhamCircleIter::new(p1, 4);
            // for p in circle {
            //     println!("{:?}", p);
            //     canvas.put(p, '*');
            // }
            // canvas.print();

            let mut canvas = Canvas::new([11, 11]);
            for (x, y) in BresenhamCircle::new(5, 5, 5) {
                print!("({}, {}), ", x, y);
                canvas.put((x, y), '*');
            }
            canvas.print();
        }
    }
}
