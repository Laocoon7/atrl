use crate::prelude::*;

pub trait FovProvider {
    fn is_opaque(&self, position: IVec2, vision_type: u8) -> bool;
}
