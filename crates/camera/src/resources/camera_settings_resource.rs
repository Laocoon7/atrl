use crate::prelude::*;

#[derive(Resource)]
pub struct CameraSettingsResource {
    pub settings: Vec<CameraSettings>,
}

impl CameraSettingsResource {
    pub fn new(settings: Vec<CameraSettings>) -> Self {
        Self { settings }
    }
}
