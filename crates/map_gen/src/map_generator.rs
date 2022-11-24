use crate::prelude::*;
pub struct MapGenerator<T> {
    map_gen_data: MapGenData<T>,
    builders: Vec<Box<dyn MapArchitect<T>>>,
}

impl<T> MapGenerator<T> {
    pub fn new(
        size: impl Size2d,
        name: &str,
        starting_position: impl Point2d,
        rng: Box<dyn RngCore>,
        starter: Box<dyn MapArchitect<T>>,
        user_data: T,
    ) -> Self {
        Self {
            builders: vec![starter],
            map_gen_data: MapGenData::new(
                size.as_uvec2(),
                name.to_string(),
                starting_position.as_uvec2(),
                rng,
                user_data,
            ),
        }
    }

    pub fn with(mut self, builder: Box<dyn MapArchitect<T>>) -> Self {
        self.builders.push(builder);
        self
    }

    pub fn generate(mut self) -> MapGenData<T> {
        info!("Generating map with {} builders", self.builders.len());
        for builder in self.builders.iter_mut() {
            builder.generate(&mut self.map_gen_data);
        }
        self.map_gen_data
    }
}
