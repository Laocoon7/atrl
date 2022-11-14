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

    pub terrain_theme_name: String,
    pub feature_theme_name: String,
    pub item_theme_name: String,

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

            terrain_theme_name: DEFAULT_TERRAIN_THEME_NAME.to_string(),
            feature_theme_name: DEFAULT_FEATURE_THEME_NAME.to_string(),
            item_theme_name: DEFAULT_ITEM_THEME_NAME.to_string(),

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

    pub fn set_terrain_theme(&mut self, theme_name: &str, tile_loader: &Res<TileLoader>) {
        self.terrain_theme_name = theme_name.to_string();
        self.update_all = true;

        if let Some(theme) = tile_loader.get_terrain_theme(theme_name) {
            for y in 0..self.size.y {
                for x in 0..self.size.x {
                    let tile_type = *self.terrain_types.get_unchecked((x, y)) as u8;

                    if let Some(tile) = theme.get_tile_definition(tile_type) {
                        self.terrain_index.set_unchecked((x, y), tile.index);
                        self.terrain_color.set_unchecked((x, y), tile.foreground_color);
                        self.terrain_background_color.set_unchecked((x, y), tile.background_color);
                        self.terrain_atlas.set_unchecked((x, y), Some(tile.atlas.clone()));
                    } else {
                        error!("Theme is missing TerrainType::{}", tile_type);
                    }
                }
            }
        }
    }

    pub fn set_feature_theme(&mut self, theme_name: &str, tile_loader: &Res<TileLoader>) {
        self.feature_theme_name = theme_name.to_string();
        self.update_all = true;

        if let Some(theme) = tile_loader.get_feature_theme(theme_name) {
            for y in 0..self.size.y {
                for x in 0..self.size.x {
                    let tile_type = *self.terrain_types.get_unchecked((x, y)) as u8;

                    if let Some(tile) = theme.get_tile_definition(tile_type) {
                        self.terrain_index.set_unchecked((x, y), tile.index);
                        self.terrain_color.set_unchecked((x, y), tile.foreground_color);
                        self.terrain_background_color.set_unchecked((x, y), tile.background_color);
                        self.terrain_atlas.set_unchecked((x, y), Some(tile.atlas.clone()));
                    } else {
                        error!("Theme is missing FeatureType::{}", tile_type);
                    }
                }
            }
        }
    }

    pub fn set_item_theme(&mut self, theme_name: &str, tile_loader: &Res<TileLoader>) {
        self.item_theme_name = theme_name.to_string();
        self.update_all = true;

        if let Some(theme) = tile_loader.get_item_theme(theme_name) {
            for y in 0..self.size.y {
                for x in 0..self.size.x {
                    let tile_type = *self.terrain_types.get_unchecked((x, y)) as u8;

                    if let Some(tile) = theme.get_tile_definition(tile_type) {
                        self.terrain_index.set_unchecked((x, y), tile.index);
                        self.terrain_color.set_unchecked((x, y), tile.foreground_color);
                        self.terrain_background_color.set_unchecked((x, y), tile.background_color);
                        self.terrain_atlas.set_unchecked((x, y), Some(tile.atlas.clone()));
                    } else {
                        error!("Theme is missing ItemType::{}", tile_type);
                    }
                }
            }
        }
    }
}
