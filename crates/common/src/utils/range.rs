use std::ops::{Add, Div, Mul, Sub};
use std::ops::{Bound::*, RangeBounds};

pub fn map_range<T: Copy>(source_value: T, from_range: (T, T), to_range: (T, T)) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to_range.0
        + (source_value - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub fn get_range_bounds<T: Copy, R: RangeBounds<T>>(
    range: R,
    lower_unbounded: T,
    upper_unbounded: T,
) -> (T, T) {
    let start = match range.start_bound() {
        Excluded(v) => *v,
        _ => lower_unbounded,
    };

    let end = match range.end_bound() {
        Included(v) | Excluded(v) => *v,
        Unbounded => upper_unbounded,
    };

    (start, end)
}
