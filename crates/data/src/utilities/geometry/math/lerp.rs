#![allow(clippy::use_self)]
pub trait Lerp {
    /// calculate the point at `percent` between `self` and `end`
    ///
    /// e.g. 10.lerp(20, 0.5) returns 15 and
    ///      10.lerp(20, 0.1) returns 11
    ///
    /// internally the values are cast as f32 and rounded before being returned
    fn lerp(self, end: Self, percent: f32) -> Self;
}
/// This method has to be separate and named differently because
/// f32::lerp already exists but is unstable
///
/// see [f32::lerp]
#[inline]
pub fn flerp(start: f32, end: f32, percent: f32) -> f32 { (end - start).mul_add(percent, start) }
macro_rules! impl_lerp {
    ($num_type: ty) => {
        impl Lerp for $num_type {
            #[inline]
            fn lerp(self, end: $num_type, percent: f32) -> $num_type {
                let start = self as f32;
                let end = end as f32;
                flerp(start, end, percent).round() as $num_type
            }
        }
    };
}
impl_lerp!(u8);
impl_lerp!(i8);
impl_lerp!(u16);
impl_lerp!(i16);
impl_lerp!(u32);
impl_lerp!(i32);
impl_lerp!(u64);
impl_lerp!(i64);
impl_lerp!(u128);
impl_lerp!(i128);
impl_lerp!(usize);
impl_lerp!(isize);
