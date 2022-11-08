use crate::{
    procgen::{MapArchitect, MapBuilder},
    *,
};
use atrl_utils::{DistanceAlg, Random, Size2d};

pub enum XStart {
    Left,
    Center,
    Right,
}

pub enum YStart {
    Top,
    Center,
    Bottom,
}

pub struct AreaStartingPosition<S> {
    x: XStart,
    y: YStart,
    phantom: std::marker::PhantomData<S>,
}

impl<S: Size2d> MapArchitect<S> for AreaStartingPosition<S> {
    fn generate(&mut self, builder: &mut MapBuilder<S>, _rng: &mut Random) { self.build(builder); }

    fn name(&self) -> &str { "AreaStartingPosition" }
}

impl<S: Size2d> AreaStartingPosition<S> {
    pub fn new(x: XStart, y: YStart) -> Box<Self> {
        Box::new(Self { x, y, phantom: std::marker::PhantomData })
    }

    fn build(&mut self, builder: &mut MapBuilder<S>) {
        let seed_x = match self.x {
            XStart::Left => 1,
            XStart::Center => (builder.grid.width() / 2) as i32,
            XStart::Right => (builder.grid.width() - 2) as i32,
        };

        let seed_y = match self.y {
            YStart::Top => 1,
            YStart::Center => (builder.grid.height() / 2) as i32,
            YStart::Bottom => (builder.grid.height() - 2) as i32,
        };

        let mut available_floors: Vec<(IVec2, f32)> = Vec::new();
        for (idx, tile) in builder.grid.iter().enumerate() {
            if tile.is_walkable() {
                if let Some(pt) = builder.grid.index_to_pt(idx) {
                    available_floors.push((
                        pt,
                        DistanceAlg::PythagorasSquared.distance2d(pt, IVec2::new(seed_x, seed_y)),
                    ));
                }
            }
        }
        if available_floors.is_empty() {
            panic!("No valid floors to start on");
        }

        available_floors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        builder.starting_position = Some(available_floors[0].0);
    }
}
