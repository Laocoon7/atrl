use crate::prelude::*;
pub trait FovAlgorithm {
    fn compute_fov(
        origin: IVec2,
        vision_type: u8,
        range: u32,
        provider: &impl FovProvider,
        receiver: &mut impl FovReceiver,
    );
}
