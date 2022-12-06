use toml::from_slice;

use crate::prelude::*;

embedded_resource!(
    SETTINGS_FILE,
    "../../../../assets/raws/settings/settings.toml"
);

#[derive(Debug, Resource, Deserialize)]
pub struct GameSettings {
    pub blah: Option<String>,
}

impl FromWorld for GameSettings {
    fn from_world(_world: &mut World) -> Self {
        let settings = Self::load::<Self>(SETTINGS_FILE);
        info!("Game Settings: {settings:?}");
        settings
    }
}

impl GameSettings {
    fn load<'a, T: serde::Deserialize<'a>>(raw_data: &'static [u8]) -> T {
        // Retrieve the raw data as an array of u8 (8-bit unsigned chars)
        match from_slice::<T>(raw_data) {
            Ok(settings) => settings,
            Err(e) => panic!("Unable to load settings: {e}"),
        }
    }
}
