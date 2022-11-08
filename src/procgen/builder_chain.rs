use super::MapBuilder;
use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// TileMap Builder Traits
////////////////////////////////////////////////////////////////////////////////

pub trait InitialMapArchitect<S: Size2d> {
    fn name(&self) -> &str;
    fn generate(&mut self, builder: &mut MapBuilder<S>, rng: &mut Random);
}

pub trait MapArchitect<S: Size2d> {
    fn name(&self) -> &str;
    fn generate(&mut self, builder: &mut MapBuilder<S>, rng: &mut Random);
}

////////////////////////////////////////////////////////////////////////////////
// BuilderChain - A builder that chains together map builders
////////////////////////////////////////////////////////////////////////////////

pub struct BuilderChain<S: Size2d> {
    pub level: u32,
    pub map_name: String,
    pub map_builder: MapBuilder<S>,
    builders: Vec<Box<dyn MapArchitect<S>>>,
    starter: Option<Box<dyn InitialMapArchitect<S>>>,
}

impl<S: Size2d> BuilderChain<S> {
    pub fn new<Str: ToString>(level: u32, size: S, name: Str) -> Self {
        Self {
            level,
            starter: None,
            builders: Vec::new(),
            map_name: name.to_string(),
            map_builder: MapBuilder::new(size, level, name),
        }
    }

    pub fn get_map(&self) -> Map { Map::from(self.map_builder.clone()) }

    pub fn start_with(mut self, starter: Box<dyn InitialMapArchitect<S>>) -> Self {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder."),
        };

        self
    }

    pub fn with(mut self, metabuilder: Box<dyn MapArchitect<S>>) -> Self {
        self.builders.push(metabuilder);

        self
    }

    pub fn generate(mut self, rng: &mut Random) -> Self {
        println!("Generating map with {} builders", self.builders.len());

        match &mut self.starter {
            None => panic!("Cannot run a map builder chain without a starting build system"),
            Some(starter) => {
                // Build the starting map
                starter.generate(&mut self.map_builder, rng);
            }
        }

        // Build additional layers in turn
        for metabuilder in self.builders.iter_mut() {
            metabuilder.generate(&mut self.map_builder, rng);
        }

        self
    }
}
