use crate::prelude::*;
pub enum Fov {
    Shadowcast,
    ShadowcastDirection(CardinalDirection),
}
impl Fov {
    pub fn compute<Range: Into<u32>>(
        &self,
        origin: impl GridPoint,
        vision_type: u8,
        range: Range,
        provider: &impl FovProvider,
        receiver: &mut impl FovReceiver,
    ) {
        let origin = origin.as_ivec2();
        let range = range.into();
        match self {
            Self::Shadowcast => Shadowcast::compute_fov(origin, vision_type, range, provider, receiver),
            Self::ShadowcastDirection(direction) => {
                Shadowcast::compute_direction(origin, vision_type, range, provider, receiver, *direction)
            },
        }
    }
}
