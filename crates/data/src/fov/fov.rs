use super::shared::FovAlgorithm;
use crate::prelude::*;

pub enum Fov {
    Adams,
    AdamsDirection(GridDirection),
    Shadowcast,
    ShadowcastDirection(CardinalDirection),
}

impl Fov {
    pub fn compute<Range: Into<u32>>(
        &self,
        origin: impl Point2d,
        vision_type: u8,
        range: Range,
        provider: &impl FovProvider,
        receiver: &mut impl FovReceiver,
    ) {
        let origin = origin.as_ivec2();
        let range = range.into();
        match self {
            Fov::Adams => AdamsFov::compute_fov(origin, vision_type, range, provider, receiver),
            Fov::AdamsDirection(direction) => AdamsFov::compute_direction(
                origin,
                vision_type,
                range,
                provider,
                receiver,
                *direction,
            ),
            Fov::Shadowcast => {
                Shadowcast::compute_fov(origin, vision_type, range, provider, receiver)
            }
            Fov::ShadowcastDirection(direction) => Shadowcast::compute_direction(
                origin,
                vision_type,
                range,
                provider,
                receiver,
                *direction,
            ),
        }
    }
}
