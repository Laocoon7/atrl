#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TilemapId {
    Bad = 0,
    Terrain = 1, // start at one so 0.5 lower can be a background layer
    Features,
    Items,
}

impl From<u64> for TilemapId {
    fn from(value: u64) -> Self {
        match value {
            1 => TilemapId::Terrain,
            2 => TilemapId::Features,
            3 => TilemapId::Items,
            _ => TilemapId::Bad,
        }
    }
}

impl From<TilemapId> for u64 {
    fn from(value: TilemapId) -> Self { value as u64 }
}

impl From<TilemapId> for f32 {
    fn from(value: TilemapId) -> Self { value as u32 as f32 }
}
