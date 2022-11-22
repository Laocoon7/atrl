/// represents the slope Y/X as a rational number
#[derive(Clone, Copy)]
pub struct Slope {
    pub y: i32,
    pub x: i32,
}

impl Slope {
    #[inline]
    pub fn new(y: i32, x: i32) -> Self {
        Self { y, x }
    }

    // this > y/x
    #[inline]
    pub const fn greater(&self, y: i32, x: i32) -> bool {
        self.y * x > self.x * y
    }

    // this >= y/x
    #[inline]
    pub const fn greater_or_equal(&self, y: i32, x: i32) -> bool {
        self.y * x >= self.x * y
    }

    // this < y/x
    #[inline]
    pub const fn _less(&self, y: i32, x: i32) -> bool {
        self.y * x < self.x * y
    }

    // this <= y/x
    #[inline]
    pub const fn less_or_equal(&self, y: i32, x: i32) -> bool {
        self.y * x <= self.x * y
    }

    // f64 y / x
    #[inline]
    pub fn value(&self) -> f64 {
        self.y as f64 / self.x as f64
    }
}
