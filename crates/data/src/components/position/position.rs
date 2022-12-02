use std::ops::{Add, AddAssign};

use crate::prelude::*;

#[derive(
    Default, Component, Reflect, FromReflect, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug,
)]
pub struct Position {
    world_position: WorldPosition,
    local_position: LocalPosition,
}

impl Position {
    #[inline(always)]
    pub const fn new(world_position: WorldPosition, local_position: LocalPosition) -> Self {
        Self {
            world_position,
            local_position,
        }
    }

    ///////////////////////////////
    /// LocalPosition
    ///////////////////////////////
    #[inline]
    pub const fn x(&self) -> u32 { self.local_position.x() }

    #[inline]
    pub const fn y(&self) -> u32 { self.local_position.y() }

    #[inline]
    pub const fn layer(&self) -> u32 { self.local_position.layer() }

    #[inline]
    pub const fn gridpoint(&self) -> UVec2 { self.local_position.gridpoint() }

    pub fn set_x(&mut self, value: u32) { self.local_position.set_x(value); }

    pub fn set_y(&mut self, value: u32) { self.local_position.set_y(value); }

    pub fn set_layer(&mut self, value: u32) { self.local_position.set_layer(value); }

    pub fn set_xy(&mut self, value: UVec2) { self.local_position.set_xy(value.x, value.y); }

    pub fn translation(&self) -> Vec3 { self.local_position.translation() }

    ///////////////////////////////
    /// WorldPosition
    ///////////////////////////////
    #[inline]
    pub const fn world_x(&self) -> i32 { self.world_position.x() }

    #[inline]
    pub const fn world_y(&self) -> i32 { self.world_position.y() }

    #[inline]
    pub const fn world_z(&self) -> i32 { self.world_position.z() }

    #[inline]
    pub const fn world_gridpoint(&self) -> IVec2 { self.world_position.gridpoint() }

    #[inline]
    pub const fn world_xyz(&self) -> IVec3 { self.world_position.xyz() }

    pub fn set_world_x(&mut self, value: i32) { self.world_position.set_x(value); }

    pub fn set_world_y(&mut self, value: i32) { self.world_position.set_y(value); }

    pub fn set_world_z(&mut self, value: i32) { self.world_position.set_z(value); }

    pub fn set_world_xy(&mut self, value: IVec2) { self.world_position.set_xy(value.x, value.y); }

    pub fn set_world_xyz(&mut self, value: IVec3) { self.world_position.set_xyz(value.x, value.y, value.z); }
}

impl Add<IVec2> for Position {
    type Output = Self;

    #[inline]
    fn add(self, rhs: IVec2) -> Self::Output {
        let mut world_x = self.world_x();
        let mut world_y = self.world_y();

        let mut local_x = self.x() as i32 + rhs.x;
        let mut local_y = self.y() as i32 + rhs.y;

        if local_x < 0 {
            world_x -= 1;
            local_x += GRID_WIDTH as i32;
        } else if local_x >= GRID_WIDTH as i32 {
            world_x += 1;
            local_x -= GRID_WIDTH as i32;
        }

        if local_y < 0 {
            world_y -= 1;
            local_y += GRID_HEIGHT as i32;
        } else if local_y >= GRID_HEIGHT as i32 {
            world_y += 1;
            local_y -= GRID_HEIGHT as i32;
        }

        Self::new(
            WorldPosition::new(world_x, world_y, self.world_z()),
            LocalPosition::new(local_x as u32, local_y as u32, self.layer()),
        )
    }
}

impl AddAssign<IVec2> for Position {
    #[inline]
    fn add_assign(&mut self, rhs: IVec2) {
        let new_x = self.x() as i32 + rhs.x;
        let new_y = self.y() as i32 + rhs.y;

        if new_x < 0 {
            self.set_world_x(self.world_x() - 1);
            self.set_x((new_x + GRID_WIDTH as i32) as u32);
        } else if new_x >= GRID_WIDTH as i32 {
            self.set_world_x(self.world_x() + 1);
            self.set_x((new_x - GRID_WIDTH as i32) as u32);
        } else {
            self.set_x(new_x as u32);
        }

        if new_y < 0 {
            self.set_world_y(self.world_y() - 1);
            self.set_y((new_y + GRID_HEIGHT as i32) as u32);
        } else if new_y >= GRID_HEIGHT as i32 {
            self.set_world_y(self.world_y() + 1);
            self.set_y((new_y - GRID_HEIGHT as i32) as u32);
        } else {
            self.set_y(new_y as u32);
        }
    }
}
