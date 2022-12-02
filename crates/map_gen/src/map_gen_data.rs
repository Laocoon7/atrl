use crate::prelude::*;

pub struct MapGenData<T> {
    pub user_data: T,

    pub size: UVec2,
    pub random: Random,
    pub exit_positions: Vec<UVec2>,

    pub grid: Grid<u32>,
    pub rooms: Vec<Rectangle>,
}

impl<T> MapGenData<T> {
    pub(crate) fn new(size: UVec2, random: Random, user_data: T) -> Self {
        Self {
            user_data,
            size,
            random,
            exit_positions: Vec::new(),
            grid: Grid::new_default(size),
            rooms: Vec::new(),
        }
    }
}
