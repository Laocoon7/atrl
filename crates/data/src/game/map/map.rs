use crate::prelude::*;

// This needs to impl FromWorld not derive reflect
#[derive(Component)]
pub struct Map {
    pub size: UVec2,
    pub random: Random,
    pub world_position: WorldPosition,

    pub update_all: bool,
    pub update_tiles: HashSet<UVec2>,
    pub visibility_map: VisibilityMap2d,

    pub actors: Grid<Option<Entity>>,

    pub item_tileset_id: u8,
    pub terrain_tileset_id: u8,
    pub feature_tileset_id: u8,

    pub item_layer_entity: Entity,
    pub terrain_layer_entity: Entity,
    pub feature_layer_entity: Entity,

    pub item_types: Grid<Vec<ItemType>>,
    pub terrain_types: Grid<TerrainType>,
    pub feature_types: Grid<FeatureType>,
}

impl Map {
    pub fn can_move_through(&self, index: impl Point2d, movement_component: &Movement) -> bool {
        let terrain = self
            .terrain_types
            .get(index)
            .map_or(MovementType::None as u8, |t| t.allowed_movement());
        let feature =
            self.feature_types.get(index).map_or(MovementType::Any as u8, |f| f.allowed_movement());

        println!(
            "T: {:?}, F: {:?}, M:{:?}, A:{:?}",
            terrain,
            feature,
            **movement_component,
            (terrain & feature & **movement_component)
        );

        (terrain & feature & **movement_component) != 0
    }

    pub fn can_see_through(&self, index: impl Point2d, vision_component: &Vision) -> bool {
        let terrain =
            self.terrain_types.get(index).map_or(VisionType::Any as u8, |t| t.vision_penetrates());
        let feature =
            self.feature_types.get(index).map_or(VisionType::Any as u8, |f| f.vision_penetrates());

        (terrain & feature & (**vision_component)) != 0
    }

    pub fn can_see_feature(&self, index: impl Point2d, vision_component: &Vision) -> bool {
        let feature =
            self.feature_types.get(index).map_or(VisionType::None as u8, |f| f.allowed_vision());

        (feature & **vision_component) != 0
    }

    pub fn set_terrain_at(&mut self, index: impl Point2d, terrain_type: TerrainType) {
        self.terrain_types.set(index, terrain_type);
        self.update_tiles.insert(index.as_uvec2());
    }

    pub fn set_feature_at(&mut self, index: impl Point2d, feature_type: FeatureType) {
        self.feature_types.set(index, feature_type);
        self.update_tiles.insert(index.as_uvec2());
    }

    pub fn has_actor(&mut self, index: impl Point2d) -> bool {
        self.actors.get(index).is_some()
    }

    pub fn add_actor(&mut self, index: impl Point2d, actor: Entity) {
        self.actors.set(index, Some(actor));
    }

    pub fn remove_actor(&mut self, index: impl Point2d) {
        self.actors.set(index, None);
    }

    pub fn get_actor(&self, index: impl Point2d) -> Option<Entity> {
        self.actors.get(index).and_then(|e| e.as_ref().copied())
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

pub struct MapPassThroughData {
    pub world_position: WorldPosition,
    pub random: Random,

    pub terrain_tileset_id: u8,
    pub feature_tileset_id: u8,
    pub item_tileset_id: u8,

    pub terrain_layer_entity: Entity,
    pub feature_layer_entity: Entity,
    pub item_layer_entity: Entity,
}

impl From<MapGenData<MapPassThroughData>> for Map {
    fn from(data: MapGenData<MapPassThroughData>) -> Self {
        let mut terrain_types = Grid::new_default(data.size);

        for y in 0..data.size.height() {
            for x in 0..data.size.width() {
                let v = *data.grid.get_unchecked((x, y));
                terrain_types.set((x, y), v.into());
            }
        }

        Self {
            size: data.size,
            world_position: data.user_data.world_position,
            random: data.user_data.random,
            terrain_types,
            feature_types: Grid::new_default(data.size),
            item_types: Grid::new_default(data.size),
            actors: Grid::new_default(data.size),
            terrain_tileset_id: data.user_data.terrain_tileset_id,
            feature_tileset_id: data.user_data.feature_tileset_id,
            item_tileset_id: data.user_data.item_tileset_id,
            terrain_layer_entity: data.user_data.terrain_layer_entity,
            feature_layer_entity: data.user_data.feature_layer_entity,
            item_layer_entity: data.user_data.item_layer_entity,
            update_tiles: HashSet::new(),
            update_all: true,
            visibility_map: VisibilityMap2d::new_default(data.size),
        }
    }
}
