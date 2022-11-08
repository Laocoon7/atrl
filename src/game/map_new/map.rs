use crate::game::prelude::*;

pub struct Map {
    /*
        tile_types: Grid<u64>,
        image_index: Grid<usize>,
        image_color: Grid<Color>,
        image_color_background: Grid<Color>,
        required_movement: Grid<Vec<MovementType>>,
        required_vision_to_see: Grid<Vec<VisionType>>,
        required_vision_to_see_through: Grid<Vec<VisionType>>,
    */
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
            update_tiles: Vec::new(),
        }
    }
}
