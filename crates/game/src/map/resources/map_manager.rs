use crate::prelude::*;

#[derive(Default, Resource)]
pub struct MapManager {
    _current_map: WorldPosition,
    loaded_maps: HashMap<WorldPosition, Entity>,
}

impl MapManager {
    pub fn new() -> Self {
        Self { _current_map: WorldPosition(IVec3::new(0, 0, 0)), loaded_maps: HashMap::new() }
    }

    pub fn get_or_generate(
        &mut self,
        commands: &mut Commands,
        game_context: &mut ResMut<GameContext>,
        tilesets: &Tilesets,
        world_position: WorldPosition,
    ) -> AtrlResult<Entity> {
        println!("MapManager::get_or_generate({:?})", world_position);

        if !game_context.is_valid_world_position(world_position) {
            return Err(AtrlError::InvalidWorldPosition(*world_position));
        }

        if let Some(entity) = self.loaded_maps.get(&world_position) {
            return Ok(*entity);
        }

        // TODO: check deserialize map from world_position
        let map_seed = game_context.map_manager_random.prht.get(
            world_position.x,
            world_position.y,
            world_position.z,
        );
        let mut random = Random::new(map_seed);
        let rng = Box::new(Pcg64::seed_from_u64(random.prng.next_u64()));

        let map_noise = game_context.map_manager_random.noise.get(
            world_position.x,
            world_position.y,
            world_position.z,
        );
        let map_noise = (map_noise + 1.0) * 0.5;
        let map_name =
            format!("Map ({}, {}, {})", world_position.x, world_position.y, world_position.z);

        let terrain_layer_entity = commands.spawn_empty().id();
        let feature_layer_entity = commands.spawn_empty().id();
        let item_layer_entity = commands.spawn_empty().id();

        let map = Map::from(Self::generate_map(
            [GRID_WIDTH, GRID_HEIGHT],
            &map_name,
            (GRID_WIDTH / 2, GRID_HEIGHT / 2),
            rng,
            MapPassThroughData {
                world_position: world_position,
                random,

                terrain_tileset_id: 0,
                feature_tileset_id: 0,
                item_tileset_id: 0,

                terrain_layer_entity,
                feature_layer_entity,
                item_layer_entity,
            },
        ));

        let tileset_count = tilesets.len() as f64 - 1.0;
        let tileset_selection = (tileset_count * map_noise).round() as u8;
        let tileset = tilesets
            .get_by_id(&tileset_selection)
            .unwrap_or_else(|| panic!("couldn't find tilemap_id: {:?}", tileset_selection));

        create_tilemap_on_entity(
            commands,
            terrain_layer_entity,
            [GRID_WIDTH, GRID_HEIGHT],
            f32::from(MapLayer::Terrain),
            tileset,
            1.0,
        );
        create_tilemap_on_entity(
            commands,
            feature_layer_entity,
            [GRID_WIDTH, GRID_HEIGHT],
            f32::from(MapLayer::Features),
            tileset,
            1.0,
        );
        create_tilemap_on_entity(
            commands,
            item_layer_entity,
            [GRID_WIDTH, GRID_HEIGHT],
            f32::from(MapLayer::Items),
            tileset,
            1.0,
        );

        commands.entity(terrain_layer_entity).insert(Name::new(format!(
            "TerrainLayer ({}, {}, {})",
            world_position.x, world_position.y, world_position.z
        )));
        commands.entity(feature_layer_entity).insert(Name::new(format!(
            "FeatureLayer ({}, {}, {})",
            world_position.x, world_position.y, world_position.z
        )));
        commands.entity(item_layer_entity).insert(Name::new(format!(
            "ItemLayer ({}, {}, {})",
            world_position.x, world_position.y, world_position.z
        )));

        let map_entity = commands
            .spawn((
                map,
                Name::new(format!(
                    "Map ({}, {}, {})",
                    world_position.x, world_position.y, world_position.z
                )),
                SpatialBundle::default(),
            ))
            .add_child(terrain_layer_entity)
            .add_child(feature_layer_entity)
            .add_child(item_layer_entity)
            .id();

        Ok(map_entity)
    }

    fn generate_map(
        size: impl Size2d,
        name: &str,
        starting_position: impl Point2d,
        rng: Box<dyn RngCore>,
        user_data: MapPassThroughData,
    ) -> MapGenData<MapPassThroughData> {
        MapGenerator::new(size, name, starting_position, rng, ScatterBuilder::new(), user_data)
            .with(CellularAutomataBuilder::new())
            .with(
                SetBuilder::new()
                    .with_rect(Rectangle::new(
                        starting_position.as_ivec2() - IVec2::new(1, 1),
                        starting_position.as_ivec2() + IVec2::new(1, 1),
                    ))
                    .set_value(0),
            )
            .with(FinalizerBuilder::new(1, 2))
            .generate()
    }

    pub fn get_terrain_map_around_point(
        &self,
        world_position: WorldPosition,
        local_position: UVec2,
        size: impl Size2d,
        q_map: Query<&Map>,
    ) -> Grid<TerrainType> {
        let mut grid = Grid::new_default(size);

        let half_size = size.as_ivec2() / 2;
        let local_position = local_position.as_ivec2();

        let left = local_position.x - half_size.x;
        let right = local_position.x + half_size.x;
        let bottom = local_position.y - half_size.y;
        let top = local_position.y + half_size.y;

        let mut index_x = 0;
        let mut written_x = 0;
        let mut index_y = 0;
        let mut written_y = 0;

        // 1
        if left < 0 && bottom < 0 {
            let sw_pos = WorldPosition(IVec3::new(
                world_position.0.x - 1,
                world_position.0.y - 1,
                world_position.0.z,
            ));
            if let Some(entity) = self.loaded_maps.get(&sw_pos) {
                if let Ok(map) = q_map.get(*entity) {
                    let start_x = GRID_WIDTH as i32 + left;
                    let end_x = GRID_WIDTH as i32;
                    let start_y = GRID_HEIGHT as i32 + bottom;
                    let end_y = GRID_HEIGHT as i32;
                    let bounds = ((start_x, start_y), (end_x, end_y));
                    grid.blit_copy(
                        (index_x, index_y),
                        &map.terrain_types,
                        bounds.0,
                        bounds.1.as_ivec2() - bounds.0.as_ivec2(),
                    );
                    written_x = end_x - start_x;
                }
            }
        }
        index_x += written_x;
        // 2
        if bottom < 0 {
            let s_pos = WorldPosition(IVec3::new(
                world_position.0.x,
                world_position.0.y - 1,
                world_position.0.z,
            ));
            if let Some(entity) = self.loaded_maps.get(&s_pos) {
                if let Ok(map) = q_map.get(*entity) {
                    let start_x = left.max(0);
                    let end_x = right.min(GRID_WIDTH as i32 - 1);
                    let start_y = GRID_HEIGHT as i32 + bottom;
                    let end_y = GRID_HEIGHT as i32;
                    let bounds = ((start_x, start_y), (end_x, end_y));
                    grid.blit_copy(
                        (index_x, index_y),
                        &map.terrain_types,
                        bounds.0,
                        bounds.1.as_ivec2() - bounds.0.as_ivec2(),
                    );
                    written_x = end_x - start_x;
                }
            }
        }
        index_x += written_x;
        // 3
        if right >= GRID_WIDTH as i32 && bottom < 0 {
            let se_pos = WorldPosition(IVec3::new(
                world_position.0.x + 1,
                world_position.0.y - 1,
                world_position.0.z,
            ));
            if let Some(entity) = self.loaded_maps.get(&se_pos) {
                if let Ok(map) = q_map.get(*entity) {
                    let start_x = 0;
                    let end_x = right - GRID_WIDTH as i32;
                    let start_y = GRID_HEIGHT as i32 + bottom;
                    let end_y = GRID_HEIGHT as i32;
                    let bounds = ((start_x, start_y), (end_x, end_y));
                    grid.blit_copy(
                        (index_x, index_y),
                        &map.terrain_types,
                        bounds.0,
                        bounds.1.as_ivec2() - bounds.0.as_ivec2(),
                    );
                    written_y = end_y - start_y;
                }
            }
        }
        index_x = 0;
        index_y += written_y;
        // 4
        if left < 0 {
            let w_pos = WorldPosition(IVec3::new(
                world_position.0.x - 1,
                world_position.0.y,
                world_position.0.z,
            ));
            if let Some(entity) = self.loaded_maps.get(&w_pos) {
                if let Ok(map) = q_map.get(*entity) {
                    let start_x = GRID_WIDTH as i32 + left;
                    let end_x = GRID_WIDTH as i32;
                    let start_y = bottom.max(0);
                    let end_y = top.min(GRID_HEIGHT as i32 - 1);
                    let bounds = ((start_x, start_y), (end_x, end_y));
                    grid.blit_copy(
                        (index_x, index_y),
                        &map.terrain_types,
                        bounds.0,
                        bounds.1.as_ivec2() - bounds.0.as_ivec2(),
                    );
                    written_x = end_x - start_x;
                }
            }
        }
        index_x += written_x;
        // 5
        let e_pos =
            WorldPosition(IVec3::new(world_position.0.x, world_position.0.y, world_position.0.z));
        if let Some(entity) = self.loaded_maps.get(&e_pos) {
            if let Ok(map) = q_map.get(*entity) {
                let start_x = left.max(0);
                let end_x = right.min(GRID_WIDTH as i32 - 1);
                let start_y = bottom.max(0);
                let end_y = top.min(GRID_HEIGHT as i32 - 1);
                let bounds = ((start_x, start_y), (end_x, end_y));
                grid.blit_copy(
                    (index_x, index_y),
                    &map.terrain_types,
                    bounds.0,
                    bounds.1.as_ivec2() - bounds.0.as_ivec2(),
                );
                written_x = end_x - start_x;
            }
        }
        index_x += written_x;
        // 6
        if right >= GRID_WIDTH as i32 {
            let e_pos = WorldPosition(IVec3::new(
                world_position.0.x + 1,
                world_position.0.y,
                world_position.0.z,
            ));
            if let Some(entity) = self.loaded_maps.get(&e_pos) {
                if let Ok(map) = q_map.get(*entity) {
                    let start_x = 0;
                    let end_x = right - GRID_WIDTH as i32;
                    let start_y = bottom.max(0);
                    let end_y = top.min(GRID_HEIGHT as i32 - 1);
                    let bounds = ((start_x, start_y), (end_x, end_y));
                    grid.blit_copy(
                        (index_x, index_y),
                        &map.terrain_types,
                        bounds.0,
                        bounds.1.as_ivec2() - bounds.0.as_ivec2(),
                    );
                    written_y = end_y - start_y;
                }
            }
        }
        index_x = 0;
        index_y += written_y;
        // 7
        if left < 0 && top >= GRID_HEIGHT as i32 {
            let nw_pos = WorldPosition(IVec3::new(
                world_position.0.x - 1,
                world_position.0.y + 1,
                world_position.0.z,
            ));
            if let Some(entity) = self.loaded_maps.get(&nw_pos) {
                if let Ok(map) = q_map.get(*entity) {
                    let start_x = GRID_WIDTH as i32 + left;
                    let end_x = GRID_WIDTH as i32;
                    let start_y = 0;
                    let end_y = top - GRID_HEIGHT as i32;
                    let bounds = ((start_x, start_y), (end_x, end_y));
                    grid.blit_copy(
                        (index_x, index_y),
                        &map.terrain_types,
                        bounds.0,
                        bounds.1.as_ivec2() - bounds.0.as_ivec2(),
                    );
                    written_x = end_x - start_x;
                }
            }
        }
        index_x += written_x;
        // 8
        if top >= GRID_HEIGHT as i32 {
            let n_pos = WorldPosition(IVec3::new(
                world_position.0.x,
                world_position.0.y + 1,
                world_position.0.z,
            ));
            if let Some(entity) = self.loaded_maps.get(&n_pos) {
                if let Ok(map) = q_map.get(*entity) {
                    let start_x = left.max(0);
                    let end_x = right.min(GRID_WIDTH as i32 - 1);
                    let start_y = 0;
                    let end_y = top - GRID_HEIGHT as i32;
                    let bounds = ((start_x, start_y), (end_x, end_y));
                    grid.blit_copy(
                        (index_x, index_y),
                        &map.terrain_types,
                        bounds.0,
                        bounds.1.as_ivec2() - bounds.0.as_ivec2(),
                    );
                    written_x = end_x - start_x;
                }
            }
        }
        index_x += written_x;
        // 9
        if right >= GRID_WIDTH as i32 && top >= GRID_HEIGHT as i32 {
            let ne_pos = WorldPosition(IVec3::new(
                world_position.0.x + 1,
                world_position.0.y + 1,
                world_position.0.z,
            ));
            if let Some(entity) = self.loaded_maps.get(&ne_pos) {
                if let Ok(map) = q_map.get(*entity) {
                    let start_x = 0i32;
                    let end_x = right - GRID_WIDTH as i32;
                    let start_y = 0;
                    let end_y = top - GRID_HEIGHT as i32;
                    let bounds = ((start_x, start_y), (end_x, end_y));
                    grid.blit_copy(
                        (index_x, index_y),
                        &map.terrain_types,
                        bounds.0,
                        bounds.1.as_ivec2() - bounds.0.as_ivec2(),
                    );
                }
            }
        }

        grid
    }
}
