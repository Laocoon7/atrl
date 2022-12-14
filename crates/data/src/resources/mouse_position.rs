use crate::prelude::*;

#[derive(Debug, Resource)]
pub struct MousePosition {
    position: Position,
    screen_coords: Vec2,
}

impl std::fmt::Display for MousePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MousePosition: {}", self.position)
    }
}

impl FromWorld for MousePosition {
    fn from_world(world: &mut World) -> Self {
        world.get_resource::<MapManagerResource>().map_or_else(
            || Self::new(Position::default()),
            |mm| {
                Self::new(Position::new(
                    mm.current_map.0,
                    LocalPosition::new(0, 0, MapLayer::UI as u32),
                ))
            },
        )
    }
}

impl MousePosition {
    #[inline(always)]
    pub const fn new(position: Position) -> Self {
        Self {
            position,
            screen_coords: Vec2::ZERO,
        }
    }

    #[inline(always)]
    pub const fn get_mouse_position(&self) -> Position { self.position }

    #[inline(always)]
    pub const fn get_screen_coords(&self) -> Vec2 { self.screen_coords }

    #[inline]
    pub fn set_mouse_position(&mut self, position: Position) { self.position = position; }

    #[inline]
    pub fn set_screen_coords(&mut self, screen_coords: Vec2) { self.screen_coords = screen_coords; }
}
