use crate::prelude::*;

#[derive(Clone, Component)]
pub struct ForegroundTile {
    pub texture_atlas: Handle<TextureAtlas>,
    pub index: usize,
    pub color: Color,
    pub bg_color: Color,
}

impl ForegroundTile {
    pub fn from_tile(tile: &Tile, theme_server: &ThemeServer, map_width: usize) -> Option<Self> {
        let color = match &tile.foreground_color {
            Some(color_def) => color_def.into(),
            None => Color::WHITE,
        };

        let bg_color = match &tile.background_color {
            Some(color_def) => color_def.into(),
            None => Color::NONE,
        };

        let index = if let Some((x, y)) = tile.xy {
            y * map_width + x
        } else if let Some(idx) = tile.index {
            idx
        } else {
            0
        };

        theme_server.get_tileset(&tile.tileset_name).map(|(handle, _tileset)| Self {
            texture_atlas: handle.clone(),
            index,
            color,
            bg_color,
        })
    }

    pub fn from_frame(frame: &Frame, theme_server: &ThemeServer, map_width: usize) -> Option<Self> {
        let color = match &frame.foreground_color {
            Some(color_def) => color_def.into(),
            None => Color::WHITE,
        };

        let bg_color = match &frame.background_color {
            Some(color_def) => color_def.into(),
            None => Color::NONE,
        };

        let index = if let Some((x, y)) = frame.xy {
            y * map_width + x
        } else if let Some(idx) = frame.index {
            idx
        } else {
            0
        };

        theme_server.get_tileset(&frame.tileset_name).map(|(handle, _tileset)| Self {
            texture_atlas: handle.clone(),
            index,
            color,
            bg_color,
        })
    }
}
