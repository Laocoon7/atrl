use crate::prelude::*;

pub trait Fov {
    fn compute_fov<Range: Into<u32>>(
        origin: impl Point2d,
        vision: VisionType,
        range: Range,
        provider: &impl FovProvider,
        receiver: &mut impl FovReceiver,
    );
}

pub trait FovProvider {
    fn is_opaque(&self, position: IVec2, vision_type: VisionType) -> bool;
}

pub trait FovReceiver {
    fn set_visible(&mut self, position: IVec2);
    fn get_visible(&self, position: IVec2) -> bool;
}
