use crate::prelude::*;

const DEFAULT_ITERATIONS: u32 = 10;
const U32_MIDDLE: u32 = u32::MAX / 2;

pub struct CellularAutomataBuilder<T> {
    rect: Option<Rectangle>,

    number_of_iterations: u32,

    _x: PhantomData<T>,
}

impl<T> CellularAutomataBuilder<T> {
    pub fn new() -> Box<Self> {
        Box::new(Self { rect: None, number_of_iterations: DEFAULT_ITERATIONS, _x: PhantomData })
    }

    pub fn with_rect(mut self, rectangle: Rectangle) -> Box<Self> {
        self.rect = Some(rectangle);
        Box::new(self)
    }

    pub fn with_iterations(mut self, number_of_iterations: u32) -> Box<Self> {
        self.number_of_iterations = number_of_iterations;
        Box::new(self)
    }

    fn count_neighbors(grid: &Grid<u32>, index: impl Point2d) -> u32 {
        let mut neighbors = 0;

        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                let value = match grid.get((x + index.x(), y + index.y())) {
                    Some(v) => *v,
                    None => continue,
                };
                if value > U32_MIDDLE {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }
}

impl<T> MapArchitect<T> for CellularAutomataBuilder<T> {
    fn generate(&mut self, data: &mut MapGenData<T>) {
        let rect = match &self.rect {
            Some(r) => *r,
            None => Rectangle::new((0i32, 0), data.size - UVec2::new(1, 1)),
        };

        if !data.grid.in_bounds(rect.min()) || !data.grid.in_bounds(rect.max()) {
            error!("CellularAutomataBuilder Rectangle{{ {}, {} }} is outside of bounds for Grid({}, {})", rect.min(), rect.max(), data.grid.width(), data.grid.height());
            return;
        }

        for _ in 0..self.number_of_iterations {
            let mut new_tiles = data.grid.clone();

            rect.for_each(|index| {
                let neighbors = Self::count_neighbors(&data.grid, index);

                if neighbors > 4 || neighbors == 0 {
                    new_tiles.set(index, u32::MAX);
                } else {
                    new_tiles.set(index, u32::MIN);
                }
            });

            data.grid = new_tiles;
        }
    }
}
