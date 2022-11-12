use crate::prelude::*;

#[derive(Default)]
pub struct AnimationBuilder {
    pub tile_type: Option<u16>,
    pub frames_per_second: Option<f32>,
    pub frames: Vec<Frame>,
}

impl AnimationBuilder {
    /// Creates a new `AnimationBuilder`
    /// The following `.set_XXX` **must** be called prior to `.build()`:
    /// `.set_tile_type()`
    /// `.frames_per_second()`
    pub fn new() -> Self { Self { tile_type: None, frames_per_second: None, frames: Vec::new() } }

    pub fn set_tile_type<T: Into<u16>>(&mut self, tile_type: T) -> &mut Self {
        self.tile_type = Some(tile_type.into());
        self
    }

    pub fn set_frames_per_second<F: Into<f32>>(&mut self, frames_per_second: F) -> &mut Self {
        self.frames_per_second = Some(frames_per_second.into());
        self
    }

    pub fn add_frame(&mut self, frame: Frame) -> &mut Self {
        self.frames.push(frame);
        self
    }

    pub fn build(self) -> Result<Animation> {
        let tile_type = match self.tile_type {
            Some(u) => u,
            None => {
                return Err(MyError::BuilderError("Animation".to_string(), "tile_type".to_string()))
            }
        };
        let frames_per_second = match self.frames_per_second {
            Some(u) => u,
            None => {
                return Err(MyError::BuilderError(
                    "Animation".to_string(),
                    "frames_per_second".to_string(),
                ))
            }
        };
        let frames = self.frames;

        Ok(Animation { tile_type, frames_per_second, frames })
    }
}
