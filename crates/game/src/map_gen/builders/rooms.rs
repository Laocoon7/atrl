use std::marker::PhantomData;

use crate::prelude::*;

pub struct RoomMapArchitect<S: Size2d> {
    phantom: PhantomData<S>,
}

impl<S: Size2d> InitialMapArchitect<S> for RoomMapArchitect<S> {
    fn generate(&mut self, builder: &mut MapBuilder<S>) {
        self.build_rooms(builder);
    }

    fn name(&self) -> &str {
        "RoomMapArchitectStarter"
    }
}

impl<S: Size2d> MapArchitect<S> for RoomMapArchitect<S> {
    fn generate(&mut self, builder: &mut MapBuilder<S>) {
        self.build_rooms(builder);
    }

    fn name(&self) -> &str {
        "RoomMapArchitect"
    }
}

impl<S: Size2d> RoomMapArchitect<S> {
    #[inline(always)]
    pub fn new() -> Box<Self> {
        Box::new(Self { phantom: PhantomData })
    }

    fn build_rooms(&mut self, builder: &mut MapBuilder<S>) {
        const MAX_ROOMS: u32 = 25;
        const MIN_SIZE: u32 = 6;
        const MAX_SIZE: u32 = 10;

        let rng = &mut builder.random;

        let mut rooms: Vec<Rectangle> = Vec::new();
        for _i in 0..MAX_ROOMS {
            let w = rng.prng.range(MIN_SIZE..=MAX_SIZE);
            let h = rng.prng.range(MIN_SIZE..=MAX_SIZE);
            let x = rng.prng.roll_dice(1, builder.size.width() - w - 1);
            let y = rng.prng.roll_dice(1, builder.size.height() - h - 1);
            let new_room = Rectangle::new((x, y), [w, h]);

            let ok = rooms.iter().all(|room| !new_room.intersects(*room));
            if ok {
                rooms.push(new_room);
            }
        }

        builder.rooms = Some(rooms);
    }
}
