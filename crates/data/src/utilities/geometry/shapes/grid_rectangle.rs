use bevy::utils::tracing::instrument::WithSubscriber;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridRectangle {
    pub min: Position,
    pub max: Position,
}

impl GridRectangle {
    #[inline]
    pub fn new(min: Position, max: Position) -> Self {
        let min = min;
        let max = max;
        Self {
            min: min.min(max),
            max: min.max(max),
        }
    }
}

impl GridRectangle {
    #[inline]
    pub const fn width(&self) -> u32 { self.max.x() - self.min.x() }

    #[inline]
    pub const fn height(&self) -> u32 { self.max.y() - self.min.y() }

    #[inline]
    pub const fn min(&self) -> IVec2 { self.min }

    #[inline]
    pub const fn max(&self) -> IVec2 { self.max }

    #[inline]
    pub fn is_square(&self) -> bool {
        self.width() == self.height()
    }
}

impl GridRectangle {
    #[inline]
    fn center(&self) -> Position { self.min.lerp(self.max, 0.5) }

    #[inline]
    fn left(&self) -> Position { self.center() }

    #[inline]
    fn right(&self) -> Position { self.center() }

    #[inline]
    fn top(&self) -> Position { self.center() }

    #[inline]
    fn bottom(&self) -> Position { self.center() }

    /// Check if this rectangle intersects another rectangle.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        // (self.min.cmple(other.max) & self.max.cmpge(other.min)).all()
        self.min.x <= other.max.x &&
            self.max.x >= other.min.x &&
            self.min.y <= other.max.y &&
            self.max.y >= other.min.y
    }

    /// Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, f: F)
    where F: FnMut(Position) {
        RectIter::new(self.min, self.max).for_each(f);
    }
}

impl Shape for GridRectangle {
    #[inline]
    fn get_count(&self) -> u32 { self.get_positions().len() as u32 }

    #[inline]
    fn contains(&self, position: Position) -> bool {
        self.min.x <= position.x() && self.max.x > position.x() && self.min.y <= position.y() && self.max.y > position.y()
    }

    #[inline]
    fn get_positions(&self) -> HashSet<Position> {
        let mut result = HashSet::new();
        for y in self.min.y..self.max.y {
            for x in self.min.x..self.max.x {
                result.insert(IVec2::new(x, y));
            }
        }
        result
    }
}

impl IntoIterator for GridRectangle {
    type IntoIter = RectIter;
    type Item = IVec2;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { RectIter::new(self.min, self.max) }
}

impl ShapeIter for GridRectangle {
    type Iterator = RectIter;

    #[inline]
    fn iter(&self) -> Self::Iterator { self.into_iter() }
}
