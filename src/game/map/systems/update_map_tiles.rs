use crate::prelude::*;

pub fn update_map_tiles(
    mut commands: Commands,
    renderer: Res<MapRenderer>,
    game_assets: Res<TextureAssets>,
    mut q_current_map: Query<&mut Map, With<CurrentMap>>,
    _keys: Res<Input<KeyCode>>,
) {
    let mut context = renderer.get_context();

    for mut map in q_current_map.iter_mut() {
        if map.update_all {
            for y in 0..map.size.height() {
                for x in 0..map.size.width() {
                    update_tile(&mut context, &map, (x, y), &game_assets.terminal8x8_atlas);
                }
            }
        } else {
            for index in map.update_tiles.iter() {
                update_tile(&mut context, &map, *index, &game_assets.terminal8x8_atlas);
            }
        }
        map.update_tiles.clear();
        map.update_all = false;
    }

    context.finalize(&mut commands);
}

fn update_tile(
    render_context: &mut RenderContext,
    map: &Map,
    index: impl Point2d,
    default_texture_atlas: &Handle<TextureAtlas>,
) {
    let terrain_type = match map.terrain_types.get(index) {
        Some(&terrain_type) => terrain_type,
        None => TerrainType::None,
    };

    let terrain_color = match map.terrain_color.get(index) {
        Some(&Some(terrain_color)) => terrain_color,
        Some(None) | None => terrain_type.into(),
    };

    let terrain_background_color = match map.terrain_background_color.get(index) {
        Some(&Some(color)) => color,
        Some(None) | None => Color::NONE,
    };

    let terrain_atlas = match map.terrain_atlas.get(index) {
        Some(Some(atlas)) => atlas.clone(),
        Some(None) | None => default_texture_atlas.clone(),
    };

    render_context.set_index(MapLayer::Terrain, index, terrain_type.into());
    render_context.set_foreground_color(MapLayer::Terrain, index, terrain_color);
    render_context.set_background_color(MapLayer::Terrain, index, terrain_background_color);
    render_context.set_atlas(MapLayer::Terrain, index, terrain_atlas);
}
