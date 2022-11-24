use crate::prelude::*;

pub trait ShapeIter {
    type Iterator;

    /// returns an iterator over all points in the shape, inclusively
    fn iter(&self) -> Self::Iterator;
}

#[derive(Debug, Clone)]
pub enum ShapeIterator {
    Rectangle(RectIter),
    Line(BresenhamLineIter),
    Circle(BresenhamCircleIter),
}
impl Iterator for ShapeIterator {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Line(i) => i.next(),
            Self::Circle(i) => i.next(),
            Self::Rectangle(i) => i.next(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ShapeIteratorExclusive {
    Line(BresenhamLineInclusiveIter),
}
impl Iterator for ShapeIteratorExclusive {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Line(i) => i.next(),
        }
    }
}
