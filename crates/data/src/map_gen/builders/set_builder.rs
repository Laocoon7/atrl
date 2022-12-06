use std::marker::PhantomData;

use crate::prelude::ShapeIter;

use crate::prelude::*;
pub struct SetBuilder<T> {
    shapes: Vec<Box<dyn Shape>>,
    value: u32,
    phantom: PhantomData<T>,
}

impl<T> SetBuilder<T> {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            shapes: Vec::new(),
            value: u32::MAX,
            phantom: PhantomData,
        })
    }

    fn with_shape(mut self, shape: impl Shape + 'static) -> Box<Self> {
        self.shapes.push(Box::new(shape));
        Box::new(self)
    }

    pub fn set_value(mut self, value: u32) -> Box<Self> {
        self.value = value;
        Box::new(self)
    }

    fn apply_shape(&mut self, shape: Box<impl Shape + ?Sized>) {

    }
}
impl<T> MapArchitect<T> for SetBuilder<T> {

    fn generate(&mut self, data: &mut MapGenData<T>) {
        if self.shapes.len() > 0 {
            loop {
                if self.shapes.len() == 0 {
                    break;
                }
                let shape = self.shapes.pop().unwrap();
                self.apply_shape(shape);
            }
        } else {
            self.apply_shape(
                Box::new(
                    GridRectangle::new(
                        Position::new(
                            data.world_position,
                            LocalPosition::ZERO,
                        ),
                        Position::new(
                            data.world_position,
                            LocalPosition::new(
                                data.size.x - 1,
                                data.size.y - 1,
                                MapLayer::Terrain as u32,
                            )
                        )
                    )
                )
            );
        }

        


    }
}


/*
Box::new(GridRectangle::new(
                    Position::new(
                        data.world_position,
                        LocalPosition::ZERO,
                    ),
                    Position::new(
                        data.world_position,
                        LocalPosition::new(
                            data.size.x - 1,
                            data.size.y - 1,
                            MapLayer::Terrain as u32,
                        )
                    )
                )) */