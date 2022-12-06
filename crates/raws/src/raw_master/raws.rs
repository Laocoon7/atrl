use ron::de::from_bytes;

use crate::prelude::*;

embedded_resource!(RAW_PLAYER_FILE, "../../../../assets/raws/player.ron");

#[derive(Debug)]
pub struct Raws {
    pub player: RawActor,
}

impl Raws {
    pub fn load_raw<'a, T: serde::Deserialize<'a>>(raw_data: &'static [u8]) -> T {
        // Retrieve the raw data as an array of u8 (8-bit unsigned chars)
        match from_bytes::<T>(raw_data) {
            Ok(template) => template,
            Err(e) => panic!("Unable to load template: {e}"),
        }
    }
}
