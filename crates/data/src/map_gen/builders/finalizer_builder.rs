use std::marker::PhantomData;

use crate::prelude::*;
pub struct FinalizerBuilder<T> {
    rect: Option<Rectangle>,
    input_min: u32,
    input_max: u32,
    min: u32,
    max: u32,
    phantom: PhantomData<T>,
}
impl<T> FinalizerBuilder<T> {
    /// `min` is the lowest allowed value on the map
    /// `max` is the highest allowed value on the map
    /// normally these should be set to
    /// min = `1` (as TerrainType 0 is TerrainType::None)
    /// max = `TerrainType::Max`
    ///
    /// eventually, map gen will change but we may want to do something like:
    /// ```
    /// // Generate a specific terrain layer (like rooms):
    /// let rooms = MapGenerator()
    ///   .with(BspBuilder()) // returns 0 for empty and 1 for wall
    ///   .with(FinalizerBuilder(TerrainType::Empty, TerrainType::Wall as u32).from(0, 1))
    ///   .generate();
    /// // Combine all the map layers:
    /// let final_map = MapGenerator()
    ///   // Copy the rooms layer over the final map at a specific rectangle
    ///   .with(MapFromLayerBuilder().add_layer(rooms).with_rect(Rectangle::new((20, 10), (30, 50))))
    ///   .generate();
    /// ```
    pub fn new(min: u32, max: u32) -> Box<Self> {
        Box::new(Self {
            rect: None,
            input_min: 0,
            input_max: u32::MAX,
            min,
            max,
            phantom: PhantomData,
        })
    }

    /// `min` is the lowest value currently on the map
    /// `max` is the highest value currently on the map
    ///
    /// many Builders set the values to between 0, 1
    /// others may set the values to between 0, u32::MAX
    /// make sure to pick the correct values being inputted
    /// based on the previous builders used.
    pub fn with_input_values(mut self, min: u32, max: u32) -> Box<Self> {
        self.input_min = min;
        self.input_max = max;
        Box::new(self)
    }

    pub fn with_rect(mut self, rectangle: Rectangle) -> Box<Self> {
        self.rect = Some(rectangle);
        Box::new(self)
    }
}
impl<T> MapArchitect<T> for FinalizerBuilder<T> {
    fn generate(&mut self, data: &mut MapGenData<T>) {
        let rect = match &self.rect {
            Some(r) => *r,
            None => Rectangle::new((0i32, 0), data.size - UVec2::new(1, 1)),
        };

        if !data.output_grid.in_bounds(rect.min()) || !data.output_grid.in_bounds(rect.max()) {
            error!(
                "MapRangeBuilder Rectangle{{ {}, {} }} is outside of bounds for Grid({}, {})",
                rect.min(),
                rect.max(),
                data.output_grid.width(),
                data.output_grid.height()
            );
            return;
        }

        rect.for_each(|v| {
            let value = *data.output_grid.get_unchecked(v);
            let new_value = map_range_u32(
                value,
                (self.input_min, self.input_max),
                (self.min, self.max + 1),
            );
            data.output_grid.set(v, new_value);
        });
    }
}
