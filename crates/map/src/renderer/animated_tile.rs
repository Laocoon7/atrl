use crate::prelude::*;

#[derive(Component, Clone)]
pub struct AnimatedTile {
    pub current_index: usize,
    pub frames_per_second: f32,
    pub elapsed_time: f64,
    pub foreground_tiles: Vec<ForegroundTile>,
}

impl AnimatedTile {
    pub fn from_animation(
        animation: &Animation,
        theme_server: &ThemeServer,
        map_width: usize,
    ) -> Option<(Self, ForegroundTile)> {
        let mut foreground_tiles = Vec::new();
        let mut first_tile = None;
        for frame in &animation.frames {
            if let Some(foreground_tile) =
                ForegroundTile::from_frame(frame, theme_server, map_width)
            {
                if first_tile.is_none() {
                    first_tile = Some(foreground_tile.clone());
                }
                foreground_tiles.push(foreground_tile);
            }
        }
        if let Some(first_tile) = first_tile {
            if !foreground_tiles.is_empty() {
                return Some((
                    Self {
                        current_index: 0,
                        frames_per_second: animation.frames_per_second,
                        elapsed_time: 0.0,
                        foreground_tiles,
                    },
                    first_tile,
                ));
            }
        }
        None
    }

    pub fn get_current_foreground_tile(&self) -> Option<&ForegroundTile> {
        let index = self.current_index;
        self.foreground_tiles.get(index)
    }
}
