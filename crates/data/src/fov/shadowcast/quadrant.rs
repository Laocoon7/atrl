use crate::prelude::*;
pub struct Quadrant<'a, 'w, 's> {
    direction: CardinalDirection,
    origin: Position,
    vision: u8,
    provider: &'a mut dyn FovProvider,
    q_blocks_vision: &'a Query<'w, 's, &'static BlocksVision>,
    receiver: &'a mut dyn FovReceiver,
}
impl<'a, 'w, 's> Quadrant<'a, 'w, 's> {
    pub fn new(
        direction: CardinalDirection,
        origin: Position,
        vision: u8,
        provider: &'a mut dyn FovProvider,
        q_blocks_vision: &'a Query<'w, 's, &'static BlocksVision>,
        receiver: &'a mut dyn FovReceiver,
    ) -> Self {
        Self {
            direction,
            origin,
            vision,
            provider,
            q_blocks_vision,
            receiver,
        }
    }

    // adjust the transform based on which direction we are scanning
    fn transform(&self, tile: IVec2) -> Position {
        let offset = match self.direction {
            CardinalDirection::North => IVec2::new(tile.y, -tile.x),
            CardinalDirection::South => IVec2::new(tile.y, tile.x),
            CardinalDirection::East => IVec2::new(tile.x, tile.y),
            CardinalDirection::West => IVec2::new(-tile.x, tile.y),
        };
        self.origin + offset
    }

    // mark this tile as visible
    pub fn set_visible(&mut self, tile: IVec2) { self.receiver.set_visible(self.transform(tile)); }

    // check if this tile is opaque
    pub fn is_opaque(&mut self, tile: IVec2) -> bool {
        self.provider.is_opaque(self.transform(tile), self.vision, self.q_blocks_vision)
    }

    pub fn is_clear(&mut self, tile: IVec2) -> bool { !self.is_opaque(tile) }
}
