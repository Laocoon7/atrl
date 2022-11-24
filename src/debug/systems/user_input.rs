use crate::prelude::*;

pub fn user_input(keys: Res<Input<KeyCode,>,>, mut my_app: ResMut<DebugUIState,>,) {
    let changed = if keys.just_pressed(KeyCode::Key1,) {
        my_app.window_visibility.hierarchy = !my_app.window_visibility.hierarchy;
        true
    } else if keys.just_pressed(KeyCode::Key2,) {
        my_app.window_visibility.resources = !my_app.window_visibility.resources;
        my_app.window_visibility.assets = !my_app.window_visibility.assets;
        true
    } else if keys.just_pressed(KeyCode::Key3,) {
        my_app.window_visibility.inspector = !my_app.window_visibility.inspector;
        true
    } else {
        false
    };

    if keys.just_pressed(KeyCode::Escape,) {
        my_app.window_visibility.overall = !my_app.window_visibility.overall;
    }

    if changed {
        my_app.update_ui();
    }
}
