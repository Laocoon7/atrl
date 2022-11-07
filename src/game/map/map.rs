use crate::game::prelude::*;

#[allow(dead_code)]
pub struct Map {
    tile_types: Grid<u64>,
    image_index: Grid<usize>,
    image_color: Grid<Color>,
    image_color_background: Grid<Color>,
    required_movement: Grid<Vec<MovementType>>,
    required_vision_to_see: Grid<Vec<VisionType>>,
    required_vision_to_see_through: Grid<Vec<VisionType>>,

    update_tiles: Vec<UVec2>,
}

#[allow(dead_code)]
impl Map {
    pub fn new(size: impl Size2d) -> Self {
        Self {
            tile_types: Grid::new_default(size),
            image_index: Grid::new_default(size),
            image_color: Grid::new_copy(size, Color::WHITE),
            image_color_background: Grid::new_copy(size, Color::BLACK),
            required_movement: Grid::new_default(size),
            required_vision_to_see: Grid::new_default(size),
            required_vision_to_see_through: Grid::new_default(size),
            update_tiles: Vec::new(),
        }
    }

    pub fn set_from_template(&mut self, index: impl Point2d, _template: &MapTileTemplate) {
        //self.tile_types.set(index, template.tile_type);
        //self.image_index.set(index, template.image_index);
        //self.image_color.set(index, template.image_color);
        //self.image_color_background.set(index, template.image_color_background);
        //self.required_movement.set(index, template.required_movement.clone());
        //self.required_vision_to_see.set(index, template.required_vision_to_see.clone());
        //self.required_vision_to_see_through.set(index,
        // template.required_vision_to_see_through.clone());
        self.update_tiles.push(index.as_uvec2());
    }

    //pub fn can_see(&mut self, index: impl Point2d, actor: &Actor) -> bool {}
    //pub fn can_see_through(&mut self, index: impl Point2d, actor: &Actor) -> bool {}
    //pub fn can_move(&mut self, index: impl Point2d, actor: &Actor) -> bool {}
}
