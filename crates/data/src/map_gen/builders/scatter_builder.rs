use std::marker::PhantomData;

use crate::prelude::*;
pub struct ScatterBuilder<T> {
    rect: Option<Rectangle>,
    phantom: PhantomData<T>,
}
impl<T> ScatterBuilder<T> {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            rect: None,
            phantom: PhantomData,
        })
    }

    pub fn with_rect(mut self, rectangle: Rectangle) -> Box<Self> {
        self.rect = Some(rectangle);
        Box::new(self)
    }
}
impl<T> MapArchitect<T> for ScatterBuilder<T> {
    fn generate(&mut self, data: &mut MapGenData<T>) {
        let rect = match &self.rect {
            Some(r) => *r,
            None => Rectangle::new((0i32, 0), data.size - UVec2::new(1, 1)),
        };

        if !data.output_grid.in_bounds(rect.min()) || !data.output_grid.in_bounds(rect.max()) {
            error!(
                "ScatterBuilder Rectangle{{ {}, {} }} is outside of bounds for Grid({}, {})",
                rect.min(),
                rect.max(),
                data.output_grid.width(),
                data.output_grid.height()
            );
            return;
        }

        rect.for_each(|v| {
            // TODO: look into different rng function. rng.gen_range() is for only 1 lookup.
            data.output_grid.set(v, data.random.prng.range(0..u32::MAX));
        });
    }
}
