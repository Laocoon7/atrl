use std::ops::Sub;

use crate::prelude::*;
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Line {
    start: IVec2,
    end: IVec2,
}

impl Line {
    pub fn new(start: Position, end: Position) -> Self {
        Self {
            start: start,
            end: end,
        }
    }

    #[inline]
    fn into_iter_exlusive(self) -> BresenhamLineIter { BresenhamLineIter::new(self.start, self.end) }
}

impl Shape for Line {
    #[inline]
    fn get_count(&self) -> u32 { self.start.sub(self.end).abs().max_element() as u32 }

    #[inline]
    fn contains(&self, position: Position) -> bool { self.get_positions().contains(&position) }

    #[inline]
    fn get_positions(&self) -> HashSet<Position> {
        let mut discovered = HashSet::new();
        let max_delta = self.start.sub(self.end).abs().max_element();
        for step in 0..=max_delta {
            let percent = if max_delta == 0 { 0.0 } else { step as f32 / max_delta as f32 };
            discovered.insert(self.start.lerp(self.end, percent));
        }
        discovered
    }
}

impl IntoIterator for Line {
    type IntoIter = BresenhamLineInclusiveIter;
    type Item = IVec2;

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
