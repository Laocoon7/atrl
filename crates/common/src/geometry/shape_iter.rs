use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum ShapeIterator {
    Line(Bresenham),
}

impl Iterator for ShapeIterator {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Line(i) => i.next(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ShapeIteratorExclusive {
    Line(BresenhamInclusive),
}

impl Iterator for ShapeIteratorExclusive {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Line(i) => i.next(),
        }
    }
}
