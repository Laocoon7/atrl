use crate::prelude::*;
#[derive(Default, Resource)]
pub struct MapManager {
    loaded_maps: HashMap<IVec3, Map>,
    pub current_map: Option<(IVec3, Map)>,
}
impl MapManager {
    pub fn new() -> Self {
        Self {
            current_map: None,
            loaded_maps: HashMap::new(),
        }
    }

    pub const fn get_current_map(&self) -> Option<&Map> {
        if let Some((_world_position, map)) = self.current_map.as_ref() {
            Some(map)
        } else {
            None
        }
    }

    pub const fn get_current_map_world_position(&self) -> Option<IVec3> {
        if let Some((world_position, _map)) = self.current_map.as_ref() {
            Some(*world_position)
        } else {
            None
        }
    }

    pub fn get_current_map_mut(&mut self) -> Option<&mut Map> {
        if let Some((_world_position, map)) = self.current_map.as_mut() {
            Some(map)
        } else {
            None
        }
    }

    fn set_current_map(&mut self, world_position: IVec3) {
        if let Some(mut new_map) = self.loaded_maps.remove(&world_position) {
            // current map -> loaded_maps
            if let Some((current_world_position, map)) = std::mem::take(&mut self.current_map) {
                // TODO: Hide current map...
                self.loaded_maps.insert(current_world_position, map);
            }

            new_map.update_all = true;
            self.current_map = Some((world_position, new_map));
        }
    }

    pub fn get(
        &mut self,
        commands: &mut Commands,
        game_context: &mut ResMut<GameContext>,
        world_position: IVec3,
        tilesets: &Tilesets,
        set_current: bool,
    ) -> &mut Map {
        // make sure the map is loaded.. deserialize/generate as necessary
        if self.get_loaded_map(world_position).is_none() &&
            self.get_serialized_map(game_context, world_position).is_none()
        {
            self.get_generated_map(commands, game_context, world_position, tilesets);
        }

        // make the map current if requested..
        if set_current {
            self.set_current_map(world_position);
        }

        // check if our map is the current map
        if let Some((current_world_position, map)) = self.current_map.as_mut() {
            if *current_world_position == world_position {
                return map;
            }
        }

        // our map is not current, but we know we loaded it..
        self.loaded_maps.get_mut(&world_position).expect("Error getting map.")
    }

    fn get_loaded_map(&mut self, world_position: IVec3) -> Option<&mut Map> {
        self.loaded_maps.get_mut(&world_position)
    }

    fn get_serialized_map(
        &mut self,
        _game_context: &mut ResMut<GameContext>,
        _world_position: IVec3,
    ) -> Option<&mut Map> {
        // TODO: lookup map data from serialized data
        None
    }

    fn get_generated_map(
        &mut self,
        commands: &mut Commands,
        game_context: &mut ResMut<GameContext>,
        world_position: IVec3,
        tilesets: &Tilesets,
    ) -> &mut Map {
        // Create the map size.
        let map_size = UVec2::new(GRID_WIDTH, GRID_HEIGHT);

        // Create a Random for the map to be generated from and then use as it's own.
        let map_seed = game_context.random.prht.get(world_position.x, world_position.y, world_position.z);
        let random = Random::new(map_seed);

        // Create the entity to hold the map.
        let map_entity = commands.spawn_empty().id();

        // Create the entities for each layer of the map which uses bevy_ecs_tilemap. - TERRAIN
        let tileset = tilesets.get_by_id(&TILESET_TERRAIN_ID).expect("Cannot find TILESET_TERRAIN_ID.");
        let terrain_layer_entity = commands
            .spawn(Name::new(format!(
                "TERRAIN ({}, {}, {})",
                world_position.x, world_position.y, world_position.z
            )))
            .id();
        create_tilemap_on_entity(
            commands,
            terrain_layer_entity,
            map_size,
            MapLayer::Terrain,
            tileset,
            1.0,
        );

        // Create the entities for each layer of the map which uses bevy_ecs_tilemap. - FEATURES
        let tileset = tilesets.get_by_id(&TILESET_FEATURES_ID).expect("Cannot find TILESET_FEATURES_ID.");
        let features_layer_entity = commands
            .spawn(Name::new(format!(
                "FEATURES ({}, {}, {})",
                world_position.x, world_position.y, world_position.z
            )))
            .id();
        create_tilemap_on_entity(
            commands,
            features_layer_entity,
            map_size,
            MapLayer::Features,
            tileset,
            1.0,
        );

        // Create the map.
        let map = self.generate_map(map_size, random, MapPassThroughData {
            world_position,
            map_entity,
            terrain_layer_entity,
            features_layer_entity,
        });

        // Build the map entity.
        commands
            .entity(map_entity)
            .insert((
                Name::new(format!(
                    "MAP ({}, {}, {})",
                    world_position.x, world_position.y, world_position.z
                )),
                // map,
                SpatialBundle::default(),
            ))
            .add_child(terrain_layer_entity)
            .add_child(features_layer_entity);

        // This map is currently loaded, add it to loaded_maps
        self.loaded_maps.insert(world_position, map);

        // Return a reference to the loaded_map
        self.loaded_maps.get_mut(&world_position).expect("This was just added...")
    }

    fn generate_map(&mut self, size: UVec2, random: Random, user_data: MapPassThroughData) -> Map {
        Map::from(MapGenerator::new(size, random, SetBuilder::new().set_value(1), user_data).generate())
    }
}
