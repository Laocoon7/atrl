use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// MapBuilder - Container to hold the state of the map being constructed
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct MapBuilder<S: Size2d> {
    pub size: S,
    pub level: u32,
    pub name: String,
    pub grid: Grid<TileType>,
    pub rooms: Option<Vec<Rectangle>>,
    pub starting_position: Option<IVec2>,
    pub spawn_list: Vec<(IVec2, String)>,
    pub corridors: Option<Vec<Vec<usize>>>,
}

impl<S: Size2d> MapBuilder<S> {
    pub fn new<Str: ToString>(size: S, level: u32, name: Str) -> Self {
        Self {
            size,
            level,
            rooms: None,
            corridors: None,
            name: name.to_string(),
            spawn_list: Vec::new(),
            starting_position: None,
            grid: Grid::new_default(size),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<S: Size2d> From<MapBuilder<S>> for Map {
    fn from(builder: MapBuilder<S>) -> Self { Self::new_with_tiles(builder.size, builder.grid) }
}
