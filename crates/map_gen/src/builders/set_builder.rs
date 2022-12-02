use crate::prelude::*;
pub struct SetBuilder<T> {
    rect: Option<Rectangle>,
    value: u32,
    phantom: PhantomData<T>,
}

impl<T> SetBuilder<T> {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            rect: None,
            value: u32::MAX,
            phantom: PhantomData,
        })
    }

    pub fn with_rect(mut self, rectangle: Rectangle) -> Box<Self> {
        self.rect = Some(rectangle);
        Box::new(self)
    }

    pub fn set_value(mut self, value: u32) -> Box<Self> {
        self.value = value;
        Box::new(self)
    }
}
impl<T> MapArchitect<T> for SetBuilder<T> {
    fn generate(&mut self, data: &mut MapGenData<T>) {
        let rect = match &self.rect {
            Some(r) => *r,
            None => Rectangle::new((0i32, 0), data.size - UVec2::new(1, 1)),
        };

        if !data.terrain_grid.in_bounds(rect.min()) || !data.terrain_grid.in_bounds(rect.max()) {
            error!(
                "SetBuilder Rectangle{{ {}, {} }} is outside of bounds for Grid({}, {})",
                rect.min(),
                rect.max(),
                data.terrain_grid.width(),
                data.terrain_grid.height()
            );
            return;
        }

        rect.for_each(|v| {
            data.terrain_grid.set(v, self.value);
        });
    }
}
