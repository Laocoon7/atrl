use crate::prelude::*;

#[derive(Component)]
pub struct Map {
    pub size: UVec2,
    pub world_position: WorldPosition,

    pub terrain_types: Grid<TerrainType>,
    pub feature_types: Grid<FeatureType>,
    pub item_types: Grid<Vec<ItemType>>,

    pub terrain_tileset_id: u8,
    pub feature_tileset_id: u8,
    pub item_tileset_id: u8,

    pub terrain_storage_entity: Option<Entity>,
    pub feature_storage_entity: Option<Entity>,
    pub item_storage_entity: Option<Entity>,

    pub update_tiles: Vec<UVec2>,
    pub update_all: bool,
}

impl Map {
    pub fn new<Tid: Into<u8>>(
        size: impl Size2d,
        world_position: WorldPosition,
        terrain_types: Grid<TerrainType>,
        feature_types: Grid<FeatureType>,
        item_types: Grid<Vec<ItemType>>,
        terrain_tileset_id: Tid,
        feature_tileset_id: Tid,
        item_tileset_id: Tid,
    ) -> Self {
        Self {
            size: size.as_uvec2(),

            world_position,

            // Terrain Layer
            terrain_types,

            // Feature Layer
            feature_types,

            // Items Layer
            item_types,

            // Id for the tileset to use
            terrain_tileset_id: terrain_tileset_id.into(),
            feature_tileset_id: feature_tileset_id.into(),
            item_tileset_id: item_tileset_id.into(),

            // Tilemap Entities.
            terrain_storage_entity: None,
            feature_storage_entity: None,
            item_storage_entity: None,

            // Internal render fields
            update_tiles: Vec::new(),
            update_all: true,
        }
    }

    pub fn can_move_through(&self, index: impl Point2d, movement_component: &Movement) -> bool {
        let terrain = match self.terrain_types.get(index) {
            Some(t) => t.allowed_movement(),
            None => MovementType::None as u8, // block any out of map stuff???
        };

        let feature = match self.feature_types.get(index) {
            Some(f) => f.allowed_movement(),
            None => MovementType::Any as u8,
        };

        println!(
            "T: {:?}, F: {:?}, M:{:?}, A:{:?}",
            terrain,
            feature,
            movement_component.movement_types,
            (terrain & feature & (movement_component.movement_types as u8))
        );

        (terrain & feature & (movement_component.movement_types as u8)) != 0
    }

    pub fn can_see_through(&self, index: impl Point2d, vision_component: &Vision) -> bool {
        let terrain = match self.terrain_types.get(index) {
            Some(t) => t.vision_penetrates(),
            None => VisionType::Any as u8,
        };

        let feature = match self.feature_types.get(index) {
            Some(f) => f.vision_penetrates(),
            None => VisionType::Any as u8,
        };

        (terrain & feature & (vision_component.vision_types as u8)) != 0
    }

    pub fn can_see_feature(&self, index: impl Point2d, vision_component: &Vision) -> bool {
        let feature = match self.feature_types.get(index) {
            Some(f) => f.allowed_vision(),
            None => VisionType::None as u8,
        };

        (feature & (vision_component.vision_types as u8)) != 0
    }
}
