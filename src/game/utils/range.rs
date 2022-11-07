use std::ops::{Bound::*, RangeBounds};

pub fn get_range_bounds<T: Copy, R: RangeBounds<T>>(
    range: R,
    lower_unbounded: T,
    upper_unbounded: T,
) -> (T, T) {
    let start;
    let end;
    if let Included(value) = range.start_bound() {
        start = *value;
    } else {
        start = lower_unbounded;
    }
    if let Included(value) = range.end_bound() {
        end = *value;
    } else {
        end = upper_unbounded;
    }

    (start, end)
}
