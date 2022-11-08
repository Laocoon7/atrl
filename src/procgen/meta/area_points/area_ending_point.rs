use crate::{
    game::prelude::TileType,
    procgen::{MapArchitect, MapBuilder},
    *,
};
use atrl_utils::{DistanceAlg, Random, Size2d};

#[allow(dead_code)]
pub enum XEnd {
    Left,
    Center,
    Right,
}

#[allow(dead_code)]
pub enum YEnd {
    Top,
    Center,
    Bottom,
}

pub struct AreaEndingPosition<S> {
    x: XEnd,
    y: YEnd,
    phantom: std::marker::PhantomData<S>,
}

impl<S: Size2d> MapArchitect<S> for AreaEndingPosition<S> {
    fn generate(&mut self, builder: &mut MapBuilder<S>, _rng: &mut Random) { self.build(builder); }

    fn name(&self) -> &str { "AreaEndingPosition" }
}

impl<S: Size2d> AreaEndingPosition<S> {
    #[allow(dead_code)]
    pub fn new(x: XEnd, y: YEnd) -> Box<AreaEndingPosition<S>> {
        Box::new(AreaEndingPosition { x, y, phantom: std::marker::PhantomData })
    }

    fn build(&mut self, builder: &mut MapBuilder<S>) {
        let seed_x = match self.x {
            XEnd::Left => 1,
            XEnd::Center => (builder.grid.width() / 2) as i32,
            XEnd::Right => (builder.grid.width() - 2) as i32,
        };

        let seed_y = match self.y {
            YEnd::Top => 1,
            YEnd::Center => (builder.grid.height() / 2) as i32,
            YEnd::Bottom => (builder.grid.height() - 2) as i32,
        };

        let mut available_floors: Vec<(usize, f32)> = Vec::new();
        for (idx, tile) in builder.grid.iter().enumerate() {
            if tile.is_walkable() {
                if let Some(pt) = builder.grid.index_to_pt(idx) {
                    available_floors.push((
                        idx,
                        DistanceAlg::PythagorasSquared.distance2d(pt, IVec2::new(seed_x, seed_y)),
                    ));
                }
            }
        }
        if available_floors.is_empty() {
            panic!("No valid floors to start on");
        }

        available_floors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        builder.grid[available_floors[0].0] = TileType::DownStairs
    }
}
