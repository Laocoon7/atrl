use crate::prelude::*;
pub struct MapRendererSettings {
    pub chunk_size: UVec2,
}
impl MapRendererSettings {
    pub fn new(chunk_size: impl Size2d) -> Self {
        Self {
            chunk_size: chunk_size.as_uvec2(),
        }
    }
}
