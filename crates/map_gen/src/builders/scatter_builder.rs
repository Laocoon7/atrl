use crate::prelude::*;

pub struct ScatterBuilder<T> {
    rect: Option<Rectangle>,
    _x: PhantomData<T>,
}

impl<T> ScatterBuilder<T> {
    pub fn new() -> Box<Self> { Box::new(Self { rect: None, _x: PhantomData }) }

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

        if !data.grid.in_bounds(rect.min()) || !data.grid.in_bounds(rect.max()) {
            error!(
                "ScatterBuilder Rectangle{{ {}, {} }} is outside of bounds for Grid({}, {})",
                rect.min(),
                rect.max(),
                data.grid.width(),
                data.grid.height()
            );
            return;
        }

        rect.for_each(|v| {
            // TODO: look into different rng function. rng.gen_range() is for only 1 lookup.
            data.grid.set(v, data.rng.gen_range(0..u32::MAX));
        });
    }
}
