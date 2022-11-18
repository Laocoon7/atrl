use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// MapBuilder - Container to hold the state of the map being constructed
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MapBuilder<S: Size2d> {
    pub size: S,
    pub world_position: WorldPosition,
    pub random: Random,
    pub name: String,
    pub terrain_grid: Grid<TerrainType>,
    pub feature_grid: Grid<FeatureType>,
    pub item_grid: Grid<ItemType>,
    pub rooms: Option<Vec<Rectangle>>,
    pub starting_position: Option<IVec2>,
    pub spawn_list: Vec<(IVec2, String)>,
    pub corridors: Option<Vec<Vec<usize>>>,
}

impl<S: Size2d> MapBuilder<S> {
    pub fn new<Str: ToString>(
        size: S,
        random: Random,
        world_position: WorldPosition,
        name: Str,
    ) -> Self {
        Self {
            size,
            world_position,
            random,
            rooms: None,
            corridors: None,
            name: name.to_string(),
            spawn_list: Vec::new(),
            starting_position: None,
            terrain_grid: Grid::new_default(size),
            feature_grid: Grid::new_default(size),
            item_grid: Grid::new_default(size),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<S: Size2d> From<MapBuilder<S>> for GameMap {
    fn from(builder: MapBuilder<S>) -> Self {
        Self::new(
            builder.size,
            builder.world_position,
            builder.random,
            builder.terrain_grid,
            Grid::new_copy(builder.size, FeatureType::None),
            Grid::new_clone(builder.size, Vec::new()),
            1,
            0,
            0,
        )
    }
}
