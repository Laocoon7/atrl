use atrl_common::prelude::grid_shapes::GridShape;

use crate::prelude::*;

#[derive(Component, Default,)]
pub struct TargetVisualizer {
    start: Option<IVec2,>,
    end: Option<IVec2,>,

    entity_list: Vec<Entity,>,
}

impl TargetVisualizer {
    pub fn new(start: impl Point2d, end: impl Point2d,) -> Self {
        Self {
            start: Some(start.as_ivec2(),), end: Some(end.as_ivec2(),), entity_list: Vec::new(),
        }
    }

    pub fn update(
        &mut self,
        commands: &mut Commands,
        tilesets: &Tilesets,
        start: impl Point2d,
        end: impl Point2d,
        color: Color,
    ) {
        let start = start.as_ivec2();
        let end = end.as_ivec2();
        self.start = Some(start,);
        self.end = Some(end,);

        // TODO: reuse entities updating position...
        self.clear(commands,);

        let Some(tileset) = tilesets.get_by_id(TILESET_CURSOR_ID) else {
            error!("Couldn't find tilemap_id: {:?}. Refusing to draw TargetVisualizer.", TILESET_CURSOR_ID);
            return;
        };

        let line = grid_shapes::Line::new(start, end,);

        for point in line.get_points() {
            self.entity_list.push(
                commands
                    .spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            color,
                            index: TILE_CURSOR_ID,
                            custom_size: Some(Vec2::ONE,),
                            ..Default::default()
                        },
                        texture_atlas: tileset.atlas().clone(),
                        transform: Transform::from_xyz(
                            (point.x) as f32 + 0.5,
                            (point.y) as f32 + 0.5,
                            f32::from(MapLayer::UI,),
                        ),
                        ..default()
                    },)
                    .id(),
            );
        }
    }

    pub fn clear(&mut self, commands: &mut Commands,) {
        self.start = None;
        self.end = None;
        for entity in self.entity_list.drain(..,) {
            commands.entity(entity,).despawn_recursive();
        }
    }

    pub fn get(&self,) -> Option<(IVec2, IVec2,),> {
        let Some(start) = self.start else {return None;};
        let Some(end) = self.end else {return None;};

        Some((start, end,),)
    }
}
