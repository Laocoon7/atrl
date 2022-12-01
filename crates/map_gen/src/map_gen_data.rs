use crate::prelude::*;

pub struct MapGenData<T> {
    pub user_data: T,

    pub size: UVec2,
    pub name: String,
    pub rng: Box<dyn RngCore>,
    pub starting_position: UVec2,
    pub exit_positions: Vec<UVec2>,

    pub grid: Grid<u32>,
    pub rooms: Vec<Rectangle>,
}

impl<T> MapGenData<T> {
    pub(crate) fn new(
        size: UVec2,
        name: String,
        starting_position: UVec2,
        rng: Box<dyn RngCore>,
        user_data: T,
    ) -> Self {
        Self {
            user_data,
            size,
            name,
            starting_position,
            rng,
            exit_positions: Vec::new(),
            grid: Grid::new_default(size),
            rooms: Vec::new(),
        }
    }
}
