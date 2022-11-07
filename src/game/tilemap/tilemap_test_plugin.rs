use crate::app_settings::*;
use crate::game::prelude::*;

pub struct TilemapTestPlugin;
impl Plugin for TilemapTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::First, draw_tilemaps)
            .add_startup_system(build_tilemap)
            .add_system(tilemap_input);
    }
}

/// Test building a new tilemap
fn build_tilemap(mut commands: Commands, default_assets: Res<DefaultAssets>) {
    // get texture_atlas_handle from default_assets for now
    let texture_atlas_handle = default_assets.texture_atlas_handle.clone();

    // Create a new Tilemap
    let tilemap = Tilemap::new(0, 80, 45, 0.0, texture_atlas_handle, &mut commands);

    // Get an editor (This should be switched to a closure for initial setup)
    let mut editor = tilemap.get_editor();

    // fill the screen
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            // Set the borders to RED '#'s
            if x == 0 || x == GRID_WIDTH - 1 || y == 0 || y == GRID_HEIGHT - 1 {
                editor.set_index(x, y, from_cp437('#'));
                editor.set_color(x, y, Color::RED);
            } else {
                // Random everything else
                let index = (Prng::entropy() % 256) as usize;
                let color =
                    Color::rgb(Prng::entropy_f32(), Prng::entropy_f32(), Prng::entropy_f32());

                // we set the texture_atlas's index
                editor.set_index(x, y, index);
                // we set the glyph's color
                editor.set_color(x, y, color);
            }
        }
    }

    // finalize consumes the object, but add's it as an entity for later retrieval
    // the editor will be consumed by "draw_tilemaps" system and removed automatically
    // technically the editor is borrow dependent on tilemap, so it's action queue is
    // passed to a context in order to drop the borrow.
    // This means the editor MUST be finalized first! HOWEVER, if we use a closure for
    // setup, this will be impossible to screw up.
    editor.finalize(&mut commands);
    tilemap.finalize(&mut commands);
}

/// test dynamically changing the tilemap
fn tilemap_input(mut commands: Commands, input: Res<Input<KeyCode>>, q_tilemaps: Query<&Tilemap>) {
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
