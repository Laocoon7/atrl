use crate::prelude::*;

pub struct ClearBuilder<T> {
    rect: Option<Rectangle>,
    _x: PhantomData<T>,
}

impl<T> ClearBuilder<T> {
    pub fn new() -> Box<Self> {
        Box::new(Self { rect: None, _x: PhantomData })
    }

    pub fn with_rect(mut self, rectangle: Rectangle) -> Box<Self> {
        self.rect = Some(rectangle);
        Box::new(self)
    }
}

impl<T> MapArchitect<T> for ClearBuilder<T> {
    fn generate(&mut self, data: &mut MapGenData<T>) {
        let rect = match &self.rect {
            Some(r) => r.clone(),
            None => Rectangle::new((0i32, 0), data.size),
        };

        if !data.grid.in_bounds(rect.min()) || !data.grid.in_bounds(rect.max()) {
            error!(
                "ClearBuilder Rectangle{{ {}, {} }} is outside of bounds for Grid({}, {})",
                rect.min(),
                rect.max(),
                data.grid.width(),
                data.grid.height()
            );
            return;
        }

        rect.for_each(|v| {
            data.grid.set(v, u32::MIN);
        });
    }
}
