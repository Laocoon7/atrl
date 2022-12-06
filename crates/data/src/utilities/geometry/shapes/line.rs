use std::fmt::Display;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Line {
    end: Position,
    start: Position,
}

impl Line {
    #[inline(always)]
    pub const fn new(start: Position, end: Position) -> Self { Self { start, end } }

    #[inline]
    fn into_iter_exlusive(self) -> BresenhamLineIter { BresenhamLineIter::new(self.start, self.end) }
}

impl Shape for Line {
    #[inline]
    fn get_count(&self) -> u32 { self.start.distance(self.end) }

    #[inline]
    fn contains(&self, position: Position) -> bool { self.get_positions().contains(&position) }

    #[inline]
    fn get_positions(&self) -> HashSet<Position> { self.iter().collect() }
}

impl IntoIterator for Line {
    type IntoIter = BresenhamLineInclusiveIter;
    type Item = Position;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { BresenhamLineInclusiveIter::new(self.start, self.end) }
}

//////////////////////////
// Inclusive of end point
//////////////////////////
impl ShapeIter for Line {
    type Iterator = BresenhamLineInclusiveIter;

    #[inline]
    fn iter(&self) -> Self::Iterator { self.into_iter() }
}

////////////////////////
// Exlusive of end point
////////////////////////
impl ShapeIterExclusive for Line {
    type ExlusiveIterator = BresenhamLineIter;

    #[inline]
    fn iter_exlusive(&self) -> Self::ExlusiveIterator { self.into_iter_exlusive() }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {{Start: {}, End: {}}}", self.start, self.end)
    }
}
