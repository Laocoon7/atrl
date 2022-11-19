use crate::prelude::*;

pub type VisibilityFlag = Bits;

pub const NONE: VisibilityFlag = 0; // 00
pub const VISIBLE: VisibilityFlag = 1 << 0; // 01
pub const OPAQUE: VisibilityFlag = 1 << 1; // 10
pub const VISIBLE_AND_OPAQUE: VisibilityFlag = VISIBLE | OPAQUE; // 11
