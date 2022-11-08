use crate::{
    prelude::*,
    procgen::{InitialMapArchitect, MapArchitect, MapBuilder},
};
use std::marker::PhantomData;

pub struct RoomMapArchitect<S>
where
    S: Size2d,
{
    phantom: PhantomData<S>,
}

impl<S> InitialMapArchitect<S> for RoomMapArchitect<S>
where
    S: Size2d,
{
    fn generate(&mut self, builder: &mut MapBuilder<S>, rng: &mut Random) {
        self.build_rooms(builder, rng);
    }

    fn name(&self) -> &str { "RoomMapArchitectStarter" }
}

impl<S> MapArchitect<S> for RoomMapArchitect<S>
where
    S: Size2d,
{
    fn generate(&mut self, builder: &mut MapBuilder<S>, rng: &mut Random) {
        self.build_rooms(builder, rng);
    }

    fn name(&self) -> &str { "RoomMapArchitect" }
}

impl<S> RoomMapArchitect<S>
where
    S: Size2d,
{
    #[inline(always)]
    pub fn new() -> Box<RoomMapArchitect<S>> { Box::new(RoomMapArchitect { phantom: PhantomData }) }

    fn build_rooms(&mut self, builder: &mut MapBuilder<S>, rng: &mut Random) {
        const MAX_ROOMS: u32 = 25;
        const MIN_SIZE: u32 = 6;
        const MAX_SIZE: u32 = 10;

        let mut rooms: Vec<Rectangle> = Vec::new();
        for _i in 0..MAX_ROOMS {
            let w = rng.prng.range(MIN_SIZE..=MAX_SIZE);
            let h = rng.prng.range(MIN_SIZE..=MAX_SIZE);
            let x = rng.prng.roll_dice(1, builder.size.width() - w - 1);
            let y = rng.prng.roll_dice(1, builder.size.height() - h - 1);
            let new_room = Rectangle::new([x, y], [w, h]);

            let ok = rooms.iter().all(|room| !new_room.intersects(*room));
            if ok {
                rooms.push(new_room);
            }
        }

        builder.rooms = Some(rooms);
    }
}
