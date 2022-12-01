use crate::prelude::*;
#[derive(
    Reflect,
    Component,
    Serialize,
    Deserialize,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Deref,
    DerefMut,
)]
#[reflect(Component)]
pub struct LocalPosition(pub UVec2);

impl From<LocalPosition> for Vec2 {
    fn from(value: LocalPosition) -> Self { Self::new(value.0.x as f32, value.0.y as f32) }
}

impl From<LocalPosition> for Transform {
    fn from(value: LocalPosition) -> Self {
        Self {
            translation: Vec3::new(value.0.x as f32 + 0.5, value.0.y as f32 + 0.5, 0.0),
            ..Default::default()
        }
    }
}
