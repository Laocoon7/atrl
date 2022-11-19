/// represents the slope Y/X as a rational number
#[derive(Clone)]
pub struct Slope {
    pub x: i32,
    pub y: i32,
}

impl Slope {
    // this > y/x
    #[inline]
    pub const fn greater(&self, y: i32, x: i32) -> bool {
        self.y * x > self.x * y
    }

    // s >= y/x
    #[inline]
    pub const fn greater_or_equal(&self, y: i32, x: i32) -> bool {
        self.y * x >= self.x * y
    }

    // s < y/x
    //pub fn less(&self, y: i32, x: i32) -> bool {
    //    self.y * x < self.x * y
    //}

    #[inline]
    pub const fn less_or_equal(&self, y: i32, x: i32) -> bool {
        self.y * x <= self.x * y
    } // this <= y/x
}
