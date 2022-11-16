use crate::prelude::*;

pub struct CameraPlugin<T> {
    camera_settings: Vec<CameraSettings>,
    state_running: T,
}

impl<T: StateNext> CameraPlugin<T> {
    pub fn new(state_running: T, camera_settings: CameraSettings) -> Self {
        Self { camera_settings: vec![camera_settings], state_running }
    }

    pub fn add_camera(mut self, settings: CameraSettings) -> Self {
        self.camera_settings.push(settings);
        self
    }
}

impl<T: StateNext> Plugin for CameraPlugin<T> {
    fn build(&self, app: &mut App) {
        let camera_settings = if self.camera_settings.is_empty() {
            #[cfg(feature = "debug")]
            warn!("CameraPlugin has no settings added. Using default camera settings.");
            vec![CameraSettings::default()]
        } else {
            let mut v = Vec::new();
            for settings in &self.camera_settings {
                v.push(settings.clone());
            }
            v
        };
        let camera_settings_resource = CameraSettingsResource::new(camera_settings);
        app.insert_resource(camera_settings_resource);

        app.add_startup_system(spawn_cameras);
    }
}
