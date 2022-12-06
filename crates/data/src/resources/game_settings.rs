use std::time::Duration;

use toml::from_slice;

use crate::prelude::*;

// TODO: embedding means we will not be able to change these values and re-serialize them for
// next time the game runs.
embedded_resource!(
    GAME_SETTINGS_FILE,
    "../../../../assets/raws/settings/settings.toml"
);

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct GameSettings {
    // Everything here needs to be Option so we can differentiate between Serialized or not. IF the settings
    // file does not have a setting defined, it will return back `None`.
    pressed_duration: Option<Duration>,
    repeat_duration: Option<Duration>,
    unsafe_duration: Option<Duration>,
}

impl FromWorld for GameSettings {
    fn from_world(_world: &mut World) -> Self {
        // create with defaults
        let mut settings = Self::new();

        // load anything serialized
        let loaded_settings = Self::load::<Self>(GAME_SETTINGS_FILE);

        // overwrite any default settings with the serialized settings
        if loaded_settings.pressed_duration.is_some() {
            settings.pressed_duration = loaded_settings.pressed_duration;
        }

        if loaded_settings.repeat_duration.is_some() {
            settings.repeat_duration = loaded_settings.repeat_duration;
        }

        if loaded_settings.unsafe_duration.is_some() {
            settings.unsafe_duration = loaded_settings.unsafe_duration;
        }

        // Return GameSettings
        info!("Game Settings: {settings:?}");
        settings
    }
}

impl GameSettings {
    fn new() -> Self {
        // as these are our defaults, every one should be Some([default_value])
        Self {
            pressed_duration: Some(Duration::from_millis(100)),
            repeat_duration: Some(Duration::from_millis(500)),
            unsafe_duration: Some(Duration::from_millis(500)),
        }
    }

    fn load<'a, T: serde::Deserialize<'a>>(raw_data: &'static [u8]) -> T {
        // Retrieve the raw data as an array of u8 (8-bit unsigned chars)
        match from_slice::<T>(raw_data) {
            Ok(settings) => settings,
            Err(e) => panic!("Unable to load settings: {e}"),
        }
    }

    // Getters:
    // As we build from defaults in `Self::new()` every `self.Option` is guaranteed to be
    // `Some([value])` so unwrap the first `Option` in the getter
    pub fn pressed_duration(&self) -> Duration { self.pressed_duration.unwrap() }

    pub fn repeat_duration(&self) -> Duration { self.repeat_duration.unwrap() }

    pub fn unsafe_duration(&self) -> Duration { self.unsafe_duration.unwrap() }

    // Setters:
    // Set a value
    pub fn set_pressed_duration(&mut self, duration: Duration) { self.pressed_duration = Some(duration); }

    pub fn set_repeat_duration(&mut self, duration: Duration) { self.repeat_duration = Some(duration); }

    pub fn set_unsafe_duration(&mut self, duration: Duration) { self.unsafe_duration = Some(duration); }
}
