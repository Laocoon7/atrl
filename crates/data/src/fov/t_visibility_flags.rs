use crate::prelude::*;

pub type TVisibilityFlag = Bits2;

pub const NONE: TVisibilityFlag = 0; // 00
pub const VISIBLE: TVisibilityFlag = 1 << 0; // 01
pub const OPAQUE: TVisibilityFlag = 1 << 1; // 10
pub const VISIBLE_AND_OPAQUE: TVisibilityFlag = VISIBLE | OPAQUE; // 11
