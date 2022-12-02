use crate::prelude::*;
#[derive(Reflect, FromReflect, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct LocalPosition(UVec3);

impl LocalPosition {
    pub fn new(x: u32, y: u32, layer: u32) -> Self { Self(UVec3::new(x, y, layer)) }

    ///////////////////////////////
    /// Getters
    ///////////////////////////////
    pub fn x(&self) -> u32 { self.0.x }

    pub fn y(&self) -> u32 { self.0.y }

    pub fn layer(&self) -> u32 { self.0.z }

    pub fn xy(&self) -> UVec2 { UVec2::new(self.x(), self.y()) }

    ///////////////////////////////
    /// Setters
    ///////////////////////////////
    pub fn set_x(&mut self, value: u32) { self.0.x = value; }

    pub fn set_y(&mut self, value: u32) { self.0.y = value; }

    pub fn set_layer(&mut self, value: u32) { self.0.z = value; }

    pub fn set_xy(&mut self, x: u32, y: u32) {
        self.set_x(x);
        self.set_y(y);
    }

    pub fn translation(&self) -> Vec3 {
        Vec3::new(
            self.x() as f32 + 0.5,
            self.y() as f32 + 0.5,
            self.layer() as f32 * 2.0 + 1.0,
        )
    }
}
