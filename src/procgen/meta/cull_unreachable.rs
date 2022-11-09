use crate::{
    prelude::*,
    procgen::{MapArchitect, MapBuilder},
};

pub struct CullUnreachable<S> {
    phantom: std::marker::PhantomData<S>,
}

impl<S> MapArchitect<S> for CullUnreachable<S>
where
    S: Size2d,
{
    fn generate(&mut self, builder: &mut MapBuilder<S>, _: &mut Random) { self.build(builder); }

    fn name(&self) -> &str { "CullUnreachable" }
}

impl<S> CullUnreachable<S>
where
    S: Size2d,
{
    #[allow(dead_code)]
    pub fn new() -> Box<Self> { Box::new(Self { phantom: std::marker::PhantomData }) }

    fn build(&mut self, builder: &mut MapBuilder<S>) {
        let mut seen = Grid::new_copy(builder.grid.size(), false);
        let player_coord = builder.starting_position.expect("No starting position");
        seen.set(player_coord, true);

        let mut to_visit = vec![player_coord];
        while let Some(current) = to_visit.pop() {
            for direction in CardinalDirection::all() {
                let neighbour_coord = current + direction.coord();
                if let Some(neighbour_cell) = builder.grid.get(neighbour_coord) {
                    if !neighbour_cell.is_wall() {
                        let seen_cell = seen.get_mut_unchecked(neighbour_coord);
                        if !*seen_cell {
                            to_visit.push(neighbour_coord);
                        }
                        *seen_cell = true;
                    }
                }
            }
        }

        for (&seen_cell, map_cell) in seen.iter().zip(builder.grid.iter_mut()) {
            if !seen_cell && map_cell.is_floor() {
                *map_cell = TerrainType::Wall;
            }
        }
    }
}
