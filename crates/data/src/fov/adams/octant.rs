use crate::prelude::*;

pub(crate) struct Octant<'a> {
    direction: GridDirection,
    origin: IVec2,
    vision_type: VisionType,
    provider: Box<&'a dyn FovProvider>,
    receiver: Box<&'a mut dyn FovReceiver>,
}

impl<'a> Octant<'a> {
    pub fn new(
        direction: GridDirection,
        origin: IVec2,
        vision: VisionType,
        provider: Box<&'a dyn FovProvider>,
        receiver: Box<&'a mut dyn FovReceiver>,
    ) -> Self {
        Self { direction, origin, vision_type: vision, provider, receiver }
    }

    fn transform(&self, tile: IVec2) -> IVec2 {
        match self.direction {
            GridDirection::North => IVec2::new(self.origin.x + tile.x, self.origin.y - tile.y),
            GridDirection::NorthEast => IVec2::new(self.origin.x + tile.y, self.origin.y - tile.x),
            GridDirection::East => IVec2::new(self.origin.x - tile.y, self.origin.y - tile.x),
            GridDirection::SouthEast => IVec2::new(self.origin.x - tile.x, self.origin.y - tile.y),
            GridDirection::South => IVec2::new(self.origin.x - tile.x, self.origin.y + tile.y),
            GridDirection::SouthWest => IVec2::new(self.origin.x - tile.y, self.origin.y + tile.x),
            GridDirection::West => IVec2::new(self.origin.x + tile.y, self.origin.y + tile.x),
            GridDirection::NorthWest => IVec2::new(self.origin.x + tile.x, self.origin.y + tile.y),
        }
    }

    pub fn is_opaque(&self, tile: IVec2) -> bool {
        self.provider.is_opaque(self.transform(tile), self.vision_type)
    }

    pub fn is_clear(&self, tile: IVec2) -> bool {
        !self.is_opaque(tile)
    }

    pub fn set_visible(&mut self, tile: IVec2) {
        self.receiver.set_visible(self.transform(tile));
    }
}
