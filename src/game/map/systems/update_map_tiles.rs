use crate::app_settings::*;
use crate::prelude::*;

pub fn update_map_tiles(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    renderer: Res<MapRenderer>,
) {
    if keys.pressed(KeyCode::Space) {
        let mut context = renderer.get_context();

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                // set the borders to RED '#' Could be skipped this time, but it doesn't
                // seem to hurt
                if x == 0 || x == GRID_WIDTH - 1 || y == 0 || y == GRID_HEIGHT - 1 {
                    context.set_index(MapLayer::Terrain, (x, y), from_cp437('#'));
                    context.set_foreground_color(MapLayer::Terrain, (x, y), Color::RED);
                } else {
                    // Random everything else
                    let index = (Prng::entropy() % 256) as usize;
                    let color =
                        Color::rgb(Prng::entropy_f32(), Prng::entropy_f32(), Prng::entropy_f32());
                    let background_color =
                        Color::rgb(Prng::entropy_f32(), Prng::entropy_f32(), Prng::entropy_f32());

                    context.set_index(MapLayer::Terrain, (x, y), index);
                    context.set_foreground_color(MapLayer::Terrain, (x, y), color);
                    context.set_background_color(MapLayer::Terrain, (x, y), background_color);
                }
            }
        }

        context.finalize(&mut commands);
    }
}
