use self::sealed::{PositionVec2Sealed, PositionVec3Sealed};
use crate::prelude::*;

/// Vector type that represents a 2D position
pub trait PositionVec2: PositionVec2Sealed {}
impl<T: PositionVec2Sealed> PositionVec2 for T {}

/// Component that represents a 2D position
pub trait AtrlPosition2: Component {
    /// Vector type that this component translates between
    type Position: PositionVec2;

    /// Get the position as a vector
    fn get(&self) -> Self::Position;

    /// Set the position from a vector
    fn set(&mut self, pos: Self::Position);
}

/// Vector type that represents a 3D position
pub trait PositionVec3: PositionVec3Sealed {}
impl<T: PositionVec3Sealed> PositionVec3 for T {}

/// Component that represents a 3D position
pub trait AtrlPosition3: Component {
    /// Vector type that this component translates between
    type Position: PositionVec3;

    /// Get the position as a vector
    fn get(&self) -> Self::Position;

    /// Set the position from a vector
    fn set(&mut self, pos: Self::Position);
}

mod sealed {
    use crate::prelude::*;
    use bevy::math::{DVec2, DVec3};

    pub trait PositionVec2Sealed {}

    impl PositionVec2Sealed for Vec2 {}
    impl PositionVec2Sealed for DVec2 {}
    impl PositionVec2Sealed for IVec2 {}
    impl PositionVec2Sealed for UVec2 {}

    pub trait PositionVec3Sealed {}
    impl PositionVec3Sealed for Vec3 {}
    impl PositionVec3Sealed for DVec3 {}
    impl PositionVec3Sealed for IVec3 {}
    impl PositionVec3Sealed for UVec3 {}
}

mod impls {
    use crate::prelude::*;

    impl AtrlPosition2 for LocalPosition {
        type Position = IVec2;
        fn get(&self) -> Self::Position { self.position }
        fn set(&mut self, pos: Self::Position) { self.position = pos }
    }

    impl AtrlPosition2 for Transform {
        type Position = Vec2;
        fn get(&self) -> Self::Position { self.translation.truncate() }
        fn set(&mut self, pos: Self::Position) {
            self.translation = pos.extend(self.translation.z);
        }
    }

    impl AtrlPosition3 for WorldPosition {
        type Position = IVec3;
        fn get(&self) -> Self::Position { self.position }
        fn set(&mut self, pos: Self::Position) { self.position = pos }
    }

    impl AtrlPosition3 for Transform {
        type Position = Vec3;
        fn get(&self) -> Self::Position { self.translation }
        fn set(&mut self, pos: Self::Position) { self.translation = pos; }
    }
}
