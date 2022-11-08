use crate::app_settings::*;
use crate::prelude::*;

pub fn update_map_tiles(
    mut commands: Commands,
    current_map: Res<CurrentMap>,
    q_tilemaps: Query<&Tilemap>,
) {
    for tilemap in q_tilemaps.iter() {
        let tilemap_id: TilemapId = tilemap.id.into();
        match tilemap_id {
            TilemapId::Terrain => draw_terrain(&current_map.0, tilemap, &mut commands),
            TilemapId::Features => draw_features(&current_map.0, tilemap, &mut commands),
            TilemapId::Items => draw_items(&current_map.0, tilemap, &mut commands),
            TilemapId::Bad => (),
        }
    }
}

fn draw_terrain(map: &Map, tilemap: &Tilemap, commands: &mut Commands) {
    let mut editor = tilemap.get_editor();

    // fill the screen
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            // set the borders to RED '#' Could be skipped this time, but it doesn't
            // seem to hurt
            if x == 0 || x == GRID_WIDTH - 1 || y == 0 || y == GRID_HEIGHT - 1 {
                editor.set_index(x, y, from_cp437('#'));
                editor.set_color(x, y, Color::RED);
            } else {
                // Random everything else
                let index = (Prng::entropy() % 256) as usize;
                let color =
                    Color::rgb(Prng::entropy_f32(), Prng::entropy_f32(), Prng::entropy_f32());

                editor.set_index(x, y, index);
                editor.set_color(x, y, color);
            }
        }
    }

    editor.finalize(commands);
}

fn draw_features(map: &Map, tilemap: &Tilemap, commands: &mut Commands) {
    let mut editor = tilemap.get_editor();

    // fill the screen
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            // set the borders to RED '#' Could be skipped this time, but it doesn't
            // seem to hurt
            if x == 0 || x == GRID_WIDTH - 1 || y == 0 || y == GRID_HEIGHT - 1 {
                editor.set_index(x, y, from_cp437('#'));
                editor.set_color(x, y, Color::RED);
            } else {
                // Random everything else
                let index = (Prng::entropy() % 256) as usize;
                let color =
                    Color::rgb(Prng::entropy_f32(), Prng::entropy_f32(), Prng::entropy_f32());

                editor.set_index(x, y, index);
                editor.set_color(x, y, color);
            }
        }
    }

    editor.finalize(commands);
}

fn draw_items(map: &Map, tilemap: &Tilemap, commands: &mut Commands) {
    let mut editor = tilemap.get_editor();

    // fill the screen
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            // set the borders to RED '#' Could be skipped this time, but it doesn't
            // seem to hurt
            if x == 0 || x == GRID_WIDTH - 1 || y == 0 || y == GRID_HEIGHT - 1 {
                editor.set_index(x, y, from_cp437('#'));
                editor.set_color(x, y, Color::RED);
            } else {
                // Random everything else
                let index = (Prng::entropy() % 256) as usize;
                let color =
                    Color::rgb(Prng::entropy_f32(), Prng::entropy_f32(), Prng::entropy_f32());

                editor.set_index(x, y, index);
                editor.set_color(x, y, color);
            }
        }
    }

    editor.finalize(commands);
}
