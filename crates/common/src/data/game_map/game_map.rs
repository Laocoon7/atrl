use crate::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct GameMap {
    pub size: UVec2,
    pub world_position: WorldPosition,
    #[reflect(ignore)] // TODO: Random impl's Serialize/Deserialize but not Reflect
    pub random: Random,

    pub terrain_types: Grid<TerrainType>,
    pub feature_types: Grid<FeatureType>,
    pub item_types: Grid<Vec<ItemType>>,

    pub actors: Grid<Option<Entity>>,

    pub terrain_tileset_id: u8,
    pub feature_tileset_id: u8,
    pub item_tileset_id: u8,

    pub terrain_layer_entity: Option<Entity>,
    pub feature_layer_entity: Option<Entity>,
    pub item_layer_entity: Option<Entity>,

    pub update_tiles: HashSet<UVec2>,
    pub update_all: bool,
}

impl GameMap {
    pub fn new<Tid: Into<u8>>(
        size: impl Size2d,
        world_position: WorldPosition,
        random: Random,
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
            random,

            // Terrain Layer
            terrain_types,

            // Feature Layer
            feature_types,

            // Items Layer
            item_types,

            // Actors on the map
            actors: Grid::new_clone(size, None),

            // Id for the tileset to use
            terrain_tileset_id: terrain_tileset_id.into(),
            feature_tileset_id: feature_tileset_id.into(),
            item_tileset_id: item_tileset_id.into(),

            // Tilemap Entities.
            terrain_layer_entity: None,
            feature_layer_entity: None,
            item_layer_entity: None,

            // Internal render fields
            update_tiles: HashSet::new(),
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

    pub fn set_terrain_at(&mut self, index: impl Point2d, terrain_type: TerrainType) {
        self.terrain_types.set(index, terrain_type);
        self.update_tiles.insert(index.as_uvec2());
    }

    pub fn set_feature_at(&mut self, index: impl Point2d, feature_type: FeatureType) {
        self.feature_types.set(index, feature_type);
        self.update_tiles.insert(index.as_uvec2());
    }

    pub fn has_actor(&mut self, index: impl Point2d) -> bool { self.actors.get(index).is_some() }

    pub fn add_actor(&mut self, index: impl Point2d, actor: Entity) {
        self.actors.set(index, Some(actor));
    }

    pub fn remove_actor(&mut self, index: impl Point2d) { self.actors.set(index, None); }

    pub fn get_actor(&self, index: impl Point2d) -> Option<Entity> {
        match self.actors.get(index) {
            Some(e) => {
                if let Some(e) = e {
                    Some(e.clone())
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn move_actor(&mut self, from: impl Point2d, to: impl Point2d) {
        if self.has_actor(to) {
            return;
        }

        if let Some(actor) = self.get_actor(from) {
            self.remove_actor(from);
            self.add_actor(to, actor);
        }
    }
}
