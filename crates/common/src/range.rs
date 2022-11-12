use std::ops::{Bound::*, RangeBounds};

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
