use crate::prelude::*;

#[derive(Inspectable, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VisionType {
    Blind = 0,
    BlackAndWhite,
    Colored,
    Infared,
    XRay,
}

impl Default for VisionType {
    fn default() -> Self { Self::Blind }
}
