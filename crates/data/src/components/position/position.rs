use std::{
    fmt::Display,
    hash::Hash,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::prelude::*;

#[derive(Default, Component, Reflect, FromReflect, Serialize, Deserialize, Clone, Copy, Debug)]
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

    #[inline(always)]
    pub fn grid_index(&self, size: UVec2) -> Option<usize> { self.gridpoint().as_index(size) }

    #[inline(always)]
    pub fn grid_index_unchecked<I: TryInto<usize>>(&self, width: I) -> usize {
        self.gridpoint().as_index_unchecked(width)
    }

    pub fn distance(&self, other: Self) -> u32 {
        let dist_x = self.distance_x(other);
        let dist_y = self.distance_y(other);

        dist_x.max(dist_y)
    }

    pub fn distance_x(&self, other: Self) -> u32 {
        ((other.world_x() * GRID_WIDTH as i32 + other.x() as i32) -
            (self.world_x() * GRID_WIDTH as i32 + self.x() as i32))
            .unsigned_abs()
    }

    pub fn distance_y(&self, other: Self) -> u32 {
        ((other.world_y() * GRID_HEIGHT as i32 + other.y() as i32) -
            (self.world_y() * GRID_HEIGHT as i32 + self.y() as i32))
            .unsigned_abs()
    }

    pub fn lerp(&self, other: Self, percent: f32) -> Self {
        let layer = self.layer();
        let (abs_self_x, abs_self_y, abs_self_z) = self.to_absolute_position();
        let (abs_other_x, abs_other_y, abs_other_z) = other.to_absolute_position();

        let lerp_x = (abs_self_x as f64 + (abs_other_x - abs_self_x) as f64 * percent as f64) as i64;
        let lerp_y = (abs_self_y as f64 + (abs_other_y - abs_self_y) as f64 * percent as f64) as i64;
        let lerp_z = (abs_self_z as f64 + (abs_other_z - abs_self_z) as f64 * percent as f64) as i32;

        Self::from_absolute_position((lerp_x, lerp_y, lerp_z), layer)
    }

    ///////////////////////////////
    /// LocalPosition
    ///////////////////////////////
    #[inline]
    pub const fn get_local_position(&self) -> LocalPosition { self.local_position }

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
    pub const fn get_world_position(&self) -> WorldPosition { self.world_position }

    #[inline]
    pub const fn world_x(&self) -> i32 { self.world_position.x() }

    #[inline]
    pub const fn world_y(&self) -> i32 { self.world_position.y() }

    #[inline]
    pub const fn world_z(&self) -> i32 { self.world_position.z() }

    #[inline]
    pub const fn world_xyz(&self) -> IVec3 { self.world_position.xyz() }

    pub fn set_world_x(&mut self, value: i32) { self.world_position.set_x(value); }

    pub fn set_world_y(&mut self, value: i32) { self.world_position.set_y(value); }

    pub fn set_world_z(&mut self, value: i32) { self.world_position.set_z(value); }

    pub fn set_world_xy(&mut self, value: IVec2) { self.world_position.set_xy(value.x, value.y); }

    pub fn set_world_xyz(&mut self, value: IVec3) { self.world_position.set_xyz(value.x, value.y, value.z); }

    ///////////////////////////////
    /// Internal
    ///////////////////////////////
    fn to_absolute_position(&self) -> (i64, i64, i32) {
        (
            (self.world_x() * GRID_WIDTH as i32 + self.x() as i32) as i64,
            (self.world_y() * GRID_HEIGHT as i32 + self.y() as i32) as i64,
            self.world_z()
        )
    }

    fn from_absolute_position(absolute_position: (i64, i64, i32), layer: u32) -> Self {
        let world_x = (absolute_position.0 / GRID_WIDTH as i64) as i32;
        let world_y = (absolute_position.1 / GRID_HEIGHT as i64) as i32;
        let world_z = absolute_position.2;

        let local_x = (absolute_position.0 % GRID_WIDTH as i64) as u32;
        let local_y = (absolute_position.1 % GRID_HEIGHT as i64) as u32;

        Self::new(
            WorldPosition::new(world_x, world_y, world_z),
            LocalPosition::new(local_x, local_y, layer)
        )
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.world_position == other.world_position && self.gridpoint() == other.gridpoint()
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.world_position.hash(state);
        self.gridpoint().hash(state);
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position{{({}, {}, {})::({}, {})}}",
            self.world_x(),
            self.world_y(),
            self.world_z(),
            self.x(),
            self.y()
        )
    }
}

// Add offset to LocalPosition
impl Add<IVec2> for Position {
    type Output = Self;

    #[inline] // TODO: Rather large for inline isn't it??
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

// Add offset to LocalPosition
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

impl Add<GridDirection> for Position {
    type Output = Self;

    fn add(self, rhs: GridDirection) -> Self::Output {
        let coord = rhs.coord();
        self + coord
    }
}

// Sub offset to LocalPosition
impl Sub<IVec2> for Position {
    type Output = Self;

    fn sub(self, rhs: IVec2) -> Self::Output {
        let mut world_x = self.world_x();
        let mut world_y = self.world_y();

        let mut local_x = self.x() as i32 - rhs.x;
        let mut local_y = self.y() as i32 - rhs.y;

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

// Sub offset to LocalPosition
impl SubAssign<IVec2> for Position {
    fn sub_assign(&mut self, rhs: IVec2) {
        let new_x = self.x() as i32 - rhs.x;
        let new_y = self.y() as i32 - rhs.y;

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