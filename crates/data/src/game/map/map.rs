use crate::prelude::*;
// This needs to impl FromWorld not derive reflect
#[derive(Component, Clone)]
pub struct Map {
    pub size: UVec2,
    pub random: Random,
    pub world_position: WorldPosition,

    pub update_all: bool,
    pub update_tiles: HashSet<UVec2>,

    pub actors: Grid<Option<Entity>>,

    pub terrain_tileset_id: u8,
    pub feature_tileset_id: u8,
    pub item_tileset_id: u8,

    pub terrain_layer_entity: Entity,
    pub feature_layer_entity: Entity,
    pub item_layer_entity: Entity,

    pub terrain_types: Grid<TerrainType>,
    pub feature_types: Grid<FeatureType>,
    pub item_types: Grid<Vec<ItemType>>,

    pub explored_tiles: HashSet<UVec2>,
}
// OPTIMIZE: All pub fn should check / convert position to usize index
// then switch to unchecked indexing grid[index] on private functions
impl Map {
    pub fn try_add_actor(
        &mut self,
        position: impl Point2d,
        actor: Entity,
        movement_type: u8,
    ) -> bool {
        if self.can_place_actor(position, movement_type) {
            self.add_actor(position, actor);
            true
        } else {
            false
        }
    }

    pub fn try_move_actor(
        &mut self,
        from: impl Point2d,
        to: impl Point2d,
        movement_type: u8,
    ) -> bool {
        if self.can_place_actor(to, movement_type) {
            if let Some(entity) = self.remove_actor(from) {
                self.add_actor(to, entity);
                return true;
            }
        }
        false
    }

    pub fn try_remove_actor(&mut self, position: impl Point2d) -> Option<Entity> {
        self.remove_actor(position)
    }

    pub fn can_place_actor(&self, position: impl Point2d, movement_type: u8) -> bool {
        self.is_walkable(position.as_ivec2(), movement_type) && !self.has_actor(position)
    }

    pub fn set_terrain_at(&mut self, position: impl Point2d, terrain_type: TerrainType) {
        self.terrain_types.set(position, terrain_type);
        self.update_tiles.insert(position.as_uvec2());
    }

    pub fn add_feature_at(&mut self, position: impl Point2d, feature_type: FeatureType) {
        self.feature_types.set(position, feature_type);
        self.update_tiles.insert(position.as_uvec2());
    }

    pub fn has_actor(&self, position: impl Point2d) -> bool {
        match self.actors.get(position) {
            Some(opt) => opt.is_some(),
            None => false,
        }
    }

    fn add_actor(&mut self, position: impl Point2d, actor: Entity) {
        self.actors.set(position, Some(actor));
    }

    fn remove_actor(&mut self, position: impl Point2d) -> Option<Entity> {
        match self.actors.set(position, None) {
            Some(opt) => opt,
            None => None,
        }
    }

    pub fn get_actor(&self, position: impl Point2d) -> Option<Entity> {
        self.actors.get(position).and_then(|e| e.as_ref().copied())
    }

    pub fn get_actor_position(&self, actor: Entity) -> Option<IVec2> {
        self.actors.enumerate().find_map(
            |(pt, e)| {
                if e.as_ref() == Some(&actor) {
                    Some(pt)
                } else {
                    None
                }
            },
        )
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
    // TODO: Explored tiles should be passed from serialized data for the map on loading, or just a
    // new HashSet pub explored_tiles: HashSet<UVec2>
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
            update_all: true,

            update_tiles: HashSet::new(),
            random: data.user_data.random,
            actors: Grid::new_default(data.size),
            world_position: data.user_data.world_position,

            terrain_types,
            item_types: Grid::new_default(data.size),
            feature_types: Grid::new_default(data.size),

            item_tileset_id: data.user_data.item_tileset_id,
            terrain_tileset_id: data.user_data.terrain_tileset_id,
            feature_tileset_id: data.user_data.feature_tileset_id,

            item_layer_entity: data.user_data.item_layer_entity,
            terrain_layer_entity: data.user_data.terrain_layer_entity,
            feature_layer_entity: data.user_data.feature_layer_entity,

            // TODO: Add explored_tiles HashSet to MapPassThroughData
            explored_tiles: HashSet::new(),
        }
    }
}
impl FovProvider for Map {
    fn is_opaque(&self, position: IVec2, vision_type: u8) -> bool {
        // Check if the player is blind
        if (vision_type & VisionType::Blind as u8) != 0 {
            return false;
        }
        // Get the vision types that can see through this terrain:
        // None by default (if there's no terrain, there's nothing to see)
        let terrain = self
            .terrain_types
            .get(position)
            .map_or(VisionType::None.as_u8(), |t| t.vision_penetrates());
        // Get the vision types that can see through this feature:
        // Any by default (if there's no feature, don't block vision)
        let feature = self
            .feature_types
            .get(position)
            .map_or(VisionType::Any.as_u8(), |f| f.vision_penetrates());
        (terrain & feature & (vision_type)) == 0
    }
}
impl PathProvider for Map {
    fn is_walkable(&self, position: IVec2, movement_type: u8) -> bool {
        // Get the movement types that can move through this terrain:
        // None by default (this provides bounds checking)
        let terrain = self
            .terrain_types
            .get(position)
            .map_or(MovementType::None.as_u8(), |t| t.allowed_movement());
        // Get the movement types that can move through this feature:
        // Any by default (if there's no feature, don't block movement)
        let feature = self
            .feature_types
            .get(position)
            .map_or(MovementType::Any.as_u8(), |f| f.allowed_movement());
        (terrain & feature & movement_type) != 0
    }

    fn cost(&self, _position: IVec2, _movement_type: u8) -> u32 { 1 }
}
