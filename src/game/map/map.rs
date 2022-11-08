use crate::prelude::*;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(
    Debug,
    Default,
    FromPrimitive,
    ToPrimitive,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum TileType {
    #[default]
    Wall,
    Floor,
    DownStairs,
}

impl TileType {
    pub fn is_wall(self) -> bool { matches!(self, TileType::Wall) }
    pub fn is_floor(self) -> bool { matches!(self, TileType::Floor) }
    pub fn is_walkable(self) -> bool { matches!(TileType::Floor, TileType::DownStairs) }
}

impl From<TileType> for u64 {
    fn from(value: TileType) -> Self {
        ToPrimitive::to_u64(&value).expect("Failed to convert `TileType` to u64")
    }
}

pub struct Map {
    /*
        image_index: Grid<usize>,
        image_color: Grid<Color>,
        image_color_background: Grid<Color>,
        required_movement: Grid<Vec<MovementType>>,
        required_vision_to_see: Grid<Vec<VisionType>>,
        required_vision_to_see_through: Grid<Vec<VisionType>>,
    */
    size: UVec2,
    tile_types: Grid<TileType>,
    update_tiles: Vec<UVec2>,
}

impl Map {
    pub fn new(size: impl Size2d) -> Self {
        Self {
            /*
                        tile_types: Grid::new_default(size),
                        image_index: Grid::new_default(size),
                        image_color: Grid::new_copy(size, Color::WHITE),
                        image_color_background: Grid::new_copy(size, Color::BLACK),
                        required_movement: Grid::new_default(size),
                        required_vision_to_see: Grid::new_default(size),
                        required_vision_to_see_through: Grid::new_default(size),
            */
            size: size.as_uvec2(),
            update_tiles: Vec::new(),
            tile_types: Grid::new_default(size),
        }
    }

    pub fn new_with_tiles(size: impl Size2d, tile_types: Grid<TileType>) -> Self {
        Self {
            size: size.as_uvec2(),
            update_tiles: Vec::new(),
            tile_types: Grid::new_default(size),
        }
    }
}
