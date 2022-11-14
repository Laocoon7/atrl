#[repr(u16)]
pub enum TileType {
    Empty,
    Floor,
    Wall,
}

impl From<TileType> for u16 {
    fn from(value: TileType) -> Self { value as u16 }
}
