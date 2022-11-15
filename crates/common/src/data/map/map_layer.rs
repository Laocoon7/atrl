#[repr(u8)]
pub enum MapLayer {
    Terrain = 1,
    Features,
    Items,
    Actors,
    Player,
}

impl From<MapLayer> for f32 {
    fn from(value: MapLayer) -> Self { value as u8 as f32 }
}
