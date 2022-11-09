use crate::app_settings::*;
use crate::game::prelude::*;

pub struct MapLoader {
    pub maps: HashMap<WorldPosition, Entity>,
}

impl MapLoader {
    pub fn get_map_entity(&self, world_position: WorldPosition) -> Option<Entity> {
        if self.maps.contains_key(&world_position) {
            Some(self.maps[&world_position])
        } else {
            None
        }
    }

    fn add_map_entity(&mut self, world_position: WorldPosition, map_entity: Entity) {
        self.maps.insert(world_position, map_entity);
    }

    pub fn change_map(
        &mut self,
        commands: &mut Commands,
        game_context: &Res<GameContext>,
        world_position: WorldPosition,
        q_current_map: Query<Entity, With<CurrentMap>>,
    ) {
        if !game_context.is_valid_world_position(world_position) {
            info!(
                "Attempted to change map to invalid world_position: ({}, {}, {})",
                world_position.position.x, world_position.position.y, world_position.position.z
            );
            return;
        }

        let map_entity = match self.get_map_entity(world_position) {
            Some(map_entity) => map_entity,
            None => {
                let map = Self::gen_map(
                    &game_context,
                    format!(
                        "({}, {}, {})",
                        world_position.position.x,
                        world_position.position.y,
                        world_position.position.z
                    )
                    .as_str(),
                    world_position,
                );
                let map_entity = commands.spawn().insert(map).insert(CurrentMap).id();
                commands.init_resource::<ChangeTheme>();

                self.add_map_entity(world_position, map_entity);
                map_entity
            }
        };

        for entity in q_current_map.iter() {
            commands.entity(entity).remove::<CurrentMap>();
        }

        commands.entity(map_entity).insert(CurrentMap);

        // TODO: Unload far away maps
    }

    fn gen_map(game_context: &Res<GameContext>, name: &str, world_position: WorldPosition) -> Map {
        let seed = game_context.get_rng().prht.get(
            world_position.position.x,
            world_position.position.y,
            world_position.position.z,
        );
        let mut random = Random::new(seed);

        let (start_x, start_y) = random_start_position(&mut random);
        let chain = BuilderChain::new([GRID_WIDTH, GRID_HEIGHT], world_position, name)
            .start_with(CellularAutomataArchitect::new())
            .with(RoomMapArchitect::new())
            .with(AreaStartingPosition::new(start_x, start_y))
            .generate(&mut game_context.get_rng());

        let map = chain.get_map();
        map
    }
}

impl Default for MapLoader {
    fn default() -> Self { Self { maps: HashMap::new() } }
}
