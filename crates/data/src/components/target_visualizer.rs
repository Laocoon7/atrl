use atrl_common::prelude::grid_shapes::GridShape;

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TargetVisualizerStyle {
    Cursor,
    Select,
    Target,
}

impl From<TargetVisualizerStyle> for usize {
    fn from(value: TargetVisualizerStyle) -> Self {
        match value {
            TargetVisualizerStyle::Cursor => TILE_UI_CURSOR_CURSOR_ID,
            TargetVisualizerStyle::Select => TILE_UI_CURSOR_SELECT_ID,
            TargetVisualizerStyle::Target => TILE_UI_CURSOR_TARGET_ID,
        }
    }
}

#[derive(Component)]
pub struct TargetVisualizer {
    color: Color,
    style: TargetVisualizerStyle,
    start: Option<IVec2>,
    end: Option<IVec2>,
    entity_list: Vec<Entity>,
}

impl Default for TargetVisualizer {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            style: TargetVisualizerStyle::Cursor,
            start: Default::default(),
            end: Default::default(),
            entity_list: Default::default(),
        }
    }
}

impl TargetVisualizer {
    pub fn new(start: UVec2, end: UVec2, style: TargetVisualizerStyle, color: Color) -> Self {
        Self {
            color,
            style,
            entity_list: Vec::new(),
            start: Some(start.as_ivec2()),
            end: Some(end.as_ivec2()),
        }
    }

    pub fn update(&mut self, commands: &mut Commands, tilesets: &Tilesets, start: UVec2, end: UVec2) {
        let start = start.as_ivec2();
        let end = end.as_ivec2();
        self.start = Some(start);
        self.end = Some(end);

        // TODO: reuse entities updating position...
        self.clear(commands);
        let Some(tileset) = tilesets.get_by_id(&TILESET_UI_ID) else {
            error!("Couldn't find tilemap_id: {:?}. Refusing to draw TargetVisualizer.", TILESET_UI_ID);
            return;
        };

        let line = grid_shapes::Line::new(start, end);
        for point in line.get_points() {
            self.entity_list.push(
                commands
                    .spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            color: self.color,
                            index: usize::from(self.style),
                            custom_size: Some(Vec2::ONE),
                            ..Default::default()
                        },
                        texture_atlas: tileset.atlas().clone(),
                        transform: Transform::from_xyz(
                            (point.x) as f32 + 0.5,
                            (point.y) as f32 + 0.5,
                            f32::from(MapLayer::UI),
                        ),
                        ..default()
                    })
                    .id(),
            );
        }
    }

    pub fn clear(&mut self, commands: &mut Commands) {
        self.start = None;
        self.end = None;

        for entity in self.entity_list.drain(..) {
            commands.entity(entity).despawn_recursive();
        }
    }

    pub const fn get(&self) -> Option<(IVec2, IVec2)> {
        let Some(start) = self.start else {return None;};
        let Some(end) = self.end else {return None;};
        Some((start, end))
    }

    pub fn set_color(&mut self, color: Color) { self.color = color; }

    pub fn set_style(&mut self, style: TargetVisualizerStyle) { self.style = style; }
}
