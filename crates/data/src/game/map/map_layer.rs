#[repr(u8)]
pub enum MapLayer {
    Terrain = 1,
    Features,
    Items,
    Actors,
    Player,
}

impl From<MapLayer> for f32 {
    fn from(value: MapLayer) -> Self {
        (value as u16 * 2 - 1) as f32
    } // spread the map layers out incase we need extra layers.
}
