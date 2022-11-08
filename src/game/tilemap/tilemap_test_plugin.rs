use crate::prelude::*;

pub struct TilemapTestPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for TilemapTestPlugin<T> {
    fn build(&self, app: &mut App) { app.add_system_to_stage(CoreStage::First, draw_tilemaps); }
}

/*
/// Test building a new tilemap
fn _build_tilemap(mut commands: Commands, game_assets: Res<GameAssets>) {
    // get texture_atlas_handle from default_assets for now
    let texture_atlas_handle = game_assets.terminal8x8_atlas.clone();

    // Create a new Tilemap
    Tilemap::build(0, 80, 45, 0.0, texture_atlas_handle, &mut commands, &mut |(x, y)| {
        if x == 0 || x == GRID_WIDTH - 1 || y == 0 || y == GRID_HEIGHT - 1 {
            (from_cp437('#'), Color::RED)
        } else {
            // Random everything else
            let index = (Prng::entropy() % 256) as usize;
            let color = Color::rgb(Prng::entropy_f32(), Prng::entropy_f32(), Prng::entropy_f32());
            (index, color)
        }
    });
}

/// test dynamically changing the tilemap
fn _tilemap_input(mut commands: Commands, input: Res<Input<KeyCode>>, q_tilemaps: Query<&Tilemap>) {
    if input.pressed(KeyCode::Space) {
        // get all Tilemap's
        for tilemap in q_tilemaps.iter() {
            // only change the tilemap we are interested in.
            // id can become a generic argument like: ID: Into<u64> so that an impl From<ENUM> for
            // u64 will allow tilemaps id'd by enums. u64's make seeds for Prng's
            // otherwise ID: PartialEq make work better
            if tilemap.id == 0 {
                // get a new editor
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
                            let color = Color::rgb(
                                Prng::entropy_f32(),
                                Prng::entropy_f32(),
                                Prng::entropy_f32(),
                            );

                            editor.set_index(x, y, index);
                            editor.set_color(x, y, color);
                        }
                    }
                }
                // we must finalize the editor in order for the changes to take effect.
                editor.finalize(&mut commands);
            }
        }
    }
}
*/
