use crate::prelude::*;

#[derive(Component)]
pub struct Map {
    /*
    image_index: Grid<usize>,
    image_color_background: Grid<Color>,
    required_movement: Grid<Vec<MovementType>>,
    required_vision_to_see: Grid<Vec<VisionType>>,
    required_vision_to_see_through: Grid<Vec<VisionType>>,
    */
    pub size: UVec2,
    pub world_position: WorldPosition,

    pub terrain_types: Grid<TerrainType>,
    pub terrain_index: Grid<usize>,
    pub terrain_color: Grid<Option<Color>>,
    pub terrain_background_color: Grid<Option<Color>>,
    pub terrain_atlas: Grid<Option<Handle<TextureAtlas>>>,

    pub feature_types: Grid<FeatureType>,
    pub item_types: Grid<Vec<ItemType>>,

    pub update_tiles: Vec<UVec2>,
    pub update_all: bool,
}

impl Map {
    pub fn new(
        size: impl Size2d,
        world_position: WorldPosition,
        terrain_types: Grid<TerrainType>,
        feature_types: Grid<FeatureType>,
        item_types: Grid<Vec<ItemType>>,
    ) -> Self {
        /*
                    tile_types: Grid::new_default(size),
                    image_index: Grid::new_default(size),
                    image_color: Grid::new_copy(size, Color::WHITE),
                    image_color_background: Grid::new_copy(size, Color::BLACK),
                    required_movement: Grid::new_default(size),
                    required_vision_to_see: Grid::new_default(size),
                    required_vision_to_see_through: Grid::new_default(size),
        */

        Self {
            size: size.as_uvec2(),
            world_position,

            // Terrain Layer
            terrain_types,
            terrain_index: Grid::new_copy(size, 0),
            terrain_color: Grid::new_copy(size, None),
            terrain_background_color: Grid::new_copy(size, None),
            terrain_atlas: Grid::new_clone(size, None),

            // Feature Layer
            feature_types,

            // Items Layer
            item_types,

            // Internal render fields
            update_tiles: Vec::new(),
            update_all: true,
        }
    }
}
