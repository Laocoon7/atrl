use crate::prelude::*;

pub(crate) struct Quadrant<'a> {
    direction: CardinalDirection,
    origin: IVec2,
    vision: VisionType,
    provider: Box<&'a dyn FovProvider>,
    receiver: Box<&'a mut dyn FovReceiver>,
}

impl<'a> Quadrant<'a> {
    pub fn new(
        direction: CardinalDirection,
        origin: IVec2,
        vision: VisionType,
        provider: Box<&'a dyn FovProvider>,
        receiver: Box<&'a mut dyn FovReceiver>,
    ) -> Self {
        Self { direction, origin, vision, provider, receiver }
    }

    // adjust the transform based on which direction we are scanning
    fn transform(&self, tile: IVec2) -> IVec2 {
        match self.direction {
            CardinalDirection::North => IVec2::new(self.origin.x + tile.y, self.origin.y - tile.x),
            CardinalDirection::South => IVec2::new(self.origin.x + tile.y, self.origin.y + tile.x),
            CardinalDirection::East => IVec2::new(self.origin.x + tile.x, self.origin.y + tile.y),
            CardinalDirection::West => IVec2::new(self.origin.x - tile.x, self.origin.y + tile.y),
        }
    }

    // mark this tile as visible
    pub fn set_visible(&mut self, tile: IVec2) {
        self.receiver.set_visible(self.transform(tile));
    }

    // check if this tile is opaque
    pub fn is_opaque(&self, tile: IVec2) -> bool {
        self.provider.is_opaque(self.transform(tile), self.vision)
    }

    pub fn is_clear(&self, tile: IVec2) -> bool {
        !self.is_opaque(tile)
    }
}
