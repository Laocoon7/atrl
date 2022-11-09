use crate::game::prelude::*;

pub fn change_map_theme(
    mut commands: Commands,
    change_theme: Res<ChangeTheme>,
    tile_loader: Res<TileLoader>,
    mut q_current_map: Query<&mut Map, With<CurrentMap>>,
) {
    for mut map in q_current_map.iter_mut() {
        if let Some(name) = &change_theme.terrain_theme_name {
            map.set_terrain_theme(name, &tile_loader);
        }

        if let Some(name) = &change_theme.feature_theme_name {
            map.set_feature_theme(name, &tile_loader);
        }

        if let Some(name) = &change_theme.item_theme_name {
            map.set_item_theme(name, &tile_loader);
        }
    }

    commands.remove_resource::<ChangeTheme>();
}
