use crate::prelude::*;
#[derive(Default, Reflect, FromReflect, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct WorldPosition(IVec3);

impl WorldPosition {
    #[inline(always)]
    pub const fn new(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }

    ///////////////////////////////
    /// Getters
    ///////////////////////////////
    #[inline]
    pub const fn x(&self) -> i32 { self.0.x }

    #[inline]
    pub const fn y(&self) -> i32 { self.0.y }

    #[inline]
    pub const fn z(&self) -> i32 { self.0.z }

    #[inline]
    pub const fn xy(&self) -> IVec2 { IVec2::new(self.x(), self.y()) }

    #[inline]
    pub const fn xyz(&self) -> IVec3 { self.0 }

    ///////////////////////////////
    /// Setters
    ///////////////////////////////
    pub fn set_x(&mut self, value: i32) { self.0.x = value; }

    pub fn set_y(&mut self, value: i32) { self.0.y = value; }

    pub fn set_z(&mut self, value: i32) { self.0.z = value; }

    pub fn set_xy(&mut self, x: i32, y: i32) {
        self.set_x(x);
        self.set_y(y);
    }

    pub fn set_xyz(&mut self, x: i32, y: i32, z: i32) {
        self.set_xy(x, y);
        self.set_z(z);
    }
}

impl Point2d for WorldPosition {
    fn x(&self) -> i32 { self.x() }

    fn y(&self) -> i32 { self.y() }
}
