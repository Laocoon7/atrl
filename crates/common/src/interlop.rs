use self::sealed::PositionVec2Sealed;
use crate::prelude::*;

mod sealed {
    use crate::prelude::*;
    use bevy::math::DVec2;

    pub trait PositionVec2Sealed {}

    impl PositionVec2Sealed for Vec2 {}
    impl PositionVec2Sealed for DVec2 {}
    impl PositionVec2Sealed for IVec2 {}
    impl PositionVec2Sealed for UVec2 {}
}

/// Vector type that represents a 2D position
pub trait PositionVec2: PositionVec2Sealed {}
impl<T: PositionVec2Sealed,> PositionVec2 for T {}

/// Component that represents a 2D position
pub trait AtrlPosition2: Component {
    /// Vector type that this component translates between
    type Position: PositionVec2;

    /// Get the position as a vector
    fn get(&self,) -> Self::Position;

    /// Set the position from a vector
    ///
    /// TODO: Change `set_value` to `set`
    ///
    /// `bevy::reflect::Reflect` conflicts with `set` method. This is a workaround.
    fn set_value(&mut self, pos: Self::Position,);
}

impl AtrlPosition2 for Transform {
    type Position = IVec2;

    fn get(&self,) -> Self::Position { self.translation.truncate().floor().as_ivec2() }

    fn set_value(&mut self, pos: Self::Position,) {
        self.translation = Vec3::new(pos.x as f32 + 0.5, pos.y as f32 + 0.5, self.translation.z,)
    }
}
