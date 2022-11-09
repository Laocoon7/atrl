use crate::{
    prelude::*,
    procgen::{InitialMapArchitect, MapArchitect, MapBuilder},
};
use smart_default::SmartDefault;

const MAX_ITERATIONS: std::ops::Range<u32> = 5..15;

#[derive(SmartDefault, Debug, Clone)]
pub struct CellularAutomataArchitect<S: Size2d> {
    #[default(10)]
    number_of_iterations: u32,
    #[default(55)]
    start_floor_percent: u32,
    randomize_iterations: bool,
    phantom: std::marker::PhantomData<S>,
}

impl<S: Size2d> InitialMapArchitect<S> for CellularAutomataArchitect<S> {
    fn generate(&mut self, builder: &mut MapBuilder<S>, rng: &mut Random) {
        self.generate(builder, rng);
    }

    fn name(&self) -> &str { "CellularAutomataArchitectStarter" }
}

impl<S: Size2d> MapArchitect<S> for CellularAutomataArchitect<S> {
    fn generate(&mut self, builder: &mut MapBuilder<S>, _rng: &mut Random) {
        println!("CellularAutomataArchitect generate");
        self.iteration(&mut builder.grid);
    }

    fn name(&self) -> &str { "CellularAutomataArchitect" }
}

impl<S: Size2d> CellularAutomataArchitect<S> {
    #[inline(always)]
    pub fn new() -> Box<Self> { Box::default() }

    #[allow(dead_code)]
    #[inline]
    pub const fn with_iterations(mut self, iterations: u32) -> Self {
        self.number_of_iterations = iterations;
        self
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn with_start_floor_percent(mut self, start_floor_percent: u32) -> Self {
        self.start_floor_percent = start_floor_percent;
        self
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn with_randomize_iterations(mut self, randomize_iterations: bool) -> Self {
        self.randomize_iterations = randomize_iterations;
        self
    }

    fn generate(&mut self, builder: &mut MapBuilder<S>, rng: &mut Random) {
        if self.randomize_iterations {
            self.number_of_iterations = rng.prng.range(MAX_ITERATIONS);
        }

        self.random_noise_map(&mut builder.grid, rng);
        for _ in 0..=self.number_of_iterations {
            self.iteration(&mut builder.grid);
        }
    }
}

impl<S: Size2d> CellularAutomataArchitect<S> {
    fn random_noise_map(&mut self, grid: &mut Grid<TerrainType>, rng: &mut Random) {
        grid.iter_mut().for_each(|t| {
            let roll = rng.prng.range(0..100);
            if roll > self.start_floor_percent {
                *t = TerrainType::Floor;
            } else {
                *t = TerrainType::Wall;
            }
        });
    }

    fn iteration(&mut self, map: &mut Grid<TerrainType>) {
        let mut new_tiles = map.clone();

        for y in 1..map.height() as i32 - 1 {
            for x in 1..map.width() as i32 - 1 {
                let neighbors = map.count_neighbors((x, y), TerrainType::Wall);

                if neighbors > 4 || neighbors == 0 {
                    new_tiles.set((x, y), TerrainType::Wall);
                } else {
                    new_tiles.set((x, y), TerrainType::Floor);
                }
            }
        }

        *map = new_tiles;
    }
}
