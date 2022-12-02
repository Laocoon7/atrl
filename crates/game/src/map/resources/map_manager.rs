use crate::prelude::*;
#[derive(Default, Resource)]
pub struct MapManager {
    pub current_map: Option<Map>,
    loaded_maps: HashMap<IVec3, Entity>,
}
impl MapManager {
    pub fn new() -> Self {
        Self {
            current_map: None,
            loaded_maps: HashMap::new(),
        }
    }

    pub const fn get_current_map(&self) -> Option<&Map> { self.current_map.as_ref() }

    pub fn get_current_map_mut(&mut self) -> Option<&mut Map> { self.current_map.as_mut() }

    pub fn get_or_generate(
        &mut self,
        commands: &mut Commands,
        game_context: &mut ResMut<GameContext>,
        tileset_name: Option<String>,
        tileset_id: Option<u8>,
        tilesets: &Tilesets,
        world_position: IVec3,
    ) -> AtrlResult<Entity> {
        info!("MapManager::get_or_generate({:?})", world_position);
        if !game_context.is_valid_world_position(world_position) {
            return Err(AtrlError::InvalidWorldPosition(world_position));
        }

        if let Some(entity) = self.loaded_maps.get(&world_position) {
            return Ok(*entity);
        }

        // TODO: check deserialize map from world_position
        let map_seed =
            game_context.map_manager_random.prht.get(world_position.x, world_position.y, world_position.z);
        let mut random = Random::new(map_seed);
        let rng = Box::new(Pcg64::seed_from_u64(random.prng.next_u64()));
        let map_name = format!(
            "Map ({}, {}, {})",
            world_position.x, world_position.y, world_position.z
        );

        let tileset = tileset_name.map_or_else(
            || {
                tileset_id.map_or_else(
                    || tilesets.get_by_id(&0).unwrap_or_else(|| panic!("Couldn't find tilemap_id: {:?}", 0)),
                    |id| {
                        tilesets
                            .get_by_id(&id)
                            .unwrap_or_else(|| panic!("Couldn't find tilemap_id: {:?}", id))
                    },
                )
            },
            |name| {
                tilesets.get_by_name(&name).unwrap_or_else(|| panic!("Couldn't find tilemap_name: {}", &name))
            },
        );

        let tileset_id = *tileset.id();
        let terrain_layer_entity = commands.spawn_empty().id();
        let feature_layer_entity = commands.spawn_empty().id();
        let item_layer_entity = commands.spawn_empty().id();

        let map = Map::from(Self::generate_map(
            [GRID_WIDTH, GRID_HEIGHT],
            &map_name,
            (GRID_WIDTH / 2, GRID_HEIGHT / 2),
            rng,
            MapPassThroughData {
                world_position,
                random,

                terrain_tileset_id: tileset_id,
                feature_tileset_id: tileset_id,
                item_tileset_id: tileset_id,

                terrain_layer_entity,
                feature_layer_entity,
                item_layer_entity,
            },
        ));

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

        self.current_map = Some(map.clone());

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
        MapGenerator::new(
            size,
            name,
            starting_position,
            rng,
            // FinalizerBuilder::new(40, 40), You probably wanted:
            ScatterBuilder::new(),
            user_data,
        )
        .with(CellularAutomataBuilder::new())
        .with(FinalizerBuilder::new(1, 2))
        .generate()

        // MapGenerator::new(
        //    size,
        //    name,
        //    starting_position,
        //    rng,
        //    // FinalizerBuilder::new(40, 40), You probably wanted:
        //    SetBuilder::new().set_value(TILE_TERRAIN_FLOOR_ID as u32),
        //    user_data,
        //)
        //.generate()

        // if Prng::from_entropy().coin() {
        //     MapGenerator::new(size, name, starting_position, rng, ScatterBuilder::new(),
        // user_data)         .with(CellularAutomataBuilder::new())
        //         .with(FinalizerBuilder::new(1, 2))
        //         .with(
        //             SetBuilder::new()
        //                 .with_rect(Rectangle::new(
        //                     starting_position.as_ivec2() - IVec2::new(1, 1),
        //                     starting_position.as_ivec2() + IVec2::new(1, 1),
        //                 ))
        //                 .set_value(1),
        //         )
        //         .generate()
        // } else {
        //     MapGenerator::new(size, name, starting_position, rng, ScatterBuilder::new(),
        // user_data)         .with(FinalizerBuilder::new(1, 4))
        //         .with(
        //             SetBuilder::new()
        //                 .with_rect(Rectangle::new(
        //                     starting_position.as_ivec2() - IVec2::new(1, 1),
        //                     starting_position.as_ivec2() + IVec2::new(1, 1),
        //                 ))
        //                 .set_value(1),
        //         )
        //         .generate()
        // }
    }
}
