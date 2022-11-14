#[repr(u8)]
pub enum ThemeType {
    Terrain,
    Feature,
    Item,
    Actor,
}

impl From<ThemeType> for u8 {
    fn from(value: ThemeType) -> Self { value as u8 }
}
