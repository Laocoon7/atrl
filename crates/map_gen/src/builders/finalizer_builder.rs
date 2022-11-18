use crate::prelude::*;

pub struct FinalizerBuilder<T> {
    rect: Option<Rectangle>,
    min: u32,
    max: u32,
    _x: PhantomData<T>,
}

impl<T> FinalizerBuilder<T> {
    pub fn new(min: u32, max: u32) -> Box<Self> {
        Box::new(Self { rect: None, min, max, _x: PhantomData })
    }

    pub fn with_rect(mut self, rectangle: Rectangle) -> Box<Self> {
        self.rect = Some(rectangle);
        Box::new(self)
    }
}

impl<T> MapArchitect<T> for FinalizerBuilder<T> {
    fn generate(&mut self, data: &mut MapGenData<T>) {
        let rect = match &self.rect {
            Some(r) => r.clone(),
            None => Rectangle::new((0i32, 0), data.size - UVec2::new(1, 1)),
        };

        if !data.grid.in_bounds(rect.min()) || !data.grid.in_bounds(rect.max()) {
            error!(
                "MapRangeBuilder Rectangle{{ {}, {} }} is outside of bounds for Grid({}, {})",
                rect.min(),
                rect.max(),
                data.grid.width(),
                data.grid.height()
            );
            return;
        }

        rect.for_each(|v| {
            let value = *data.grid.get_unchecked(v);
            data.grid.set(v, map_range(value, (0, u32::MAX), (self.min, self.max)));
        });
    }
}
