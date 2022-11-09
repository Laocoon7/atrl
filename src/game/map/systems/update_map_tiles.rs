use crate::prelude::*;

pub fn update_map_tiles(
    mut commands: Commands,
    renderer: Res<MapRenderer>,
    tile_loader: Res<TileLoader>,
    mut q_current_map: Query<&mut Map, With<CurrentMap>>,
) {
    let mut context = renderer.get_context();

    for mut map in q_current_map.iter_mut() {
        let terrain_theme = tile_loader.get_terrain_theme_or_default(&map.terrain_theme_name);
        let feature_theme = tile_loader.get_terrain_theme_or_default(&map.feature_theme_name);
        let item_theme = tile_loader.get_terrain_theme_or_default(&map.item_theme_name);

        if map.update_all {
            for y in 0..map.size.height() {
                for x in 0..map.size.width() {
                    update_tile(
                        &mut context,
                        &map,
                        (x, y),
                        terrain_theme,
                        feature_theme,
                        item_theme,
                    );
                }
            }
        } else {
            for index in map.update_tiles.iter() {
                update_tile(&mut context, &map, *index, terrain_theme, feature_theme, item_theme);
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
    terrain_theme: &Theme,
    feature_theme: &Theme,
    item_theme: &Theme,
) {
    // TERRAIN

    let terrain_type = match map.terrain_types.get(index) {
        Some(&terrain_type) => terrain_type,
        None => TerrainType::None,
    };

    if let Some(tile) = terrain_theme.get_tile_definition(terrain_type.into()) {
        let terrain_index = tile.index;
        let terrain_color = match tile.foreground_color {
            Some(color) => color,
            None => Color::WHITE,
        };

        let terrain_background_color = match tile.background_color {
            Some(color) => color,
            None => Color::NONE,
        };

        let terrain_atlas = tile.atlas.clone();

        render_context.set_atlas(MapLayer::Terrain, index, terrain_atlas);
        render_context.set_index(MapLayer::Terrain, index, terrain_index);
        render_context.set_foreground_color(MapLayer::Terrain, index, terrain_color);
        render_context.set_background_color(MapLayer::Terrain, index, terrain_background_color);
    } else {
        error!("Theme is missing TerrainType::{}", u8::from(terrain_type));
    }

    // FEATURES

    let feature_type = match map.feature_types.get(index) {
        Some(&feature_type) => feature_type,
        None => FeatureType::None,
    };

    if let Some(tile) = feature_theme.get_tile_definition(feature_type.into()) {
        let feature_index = tile.index;
        let feature_color = match tile.foreground_color {
            Some(color) => color,
            None => Color::WHITE,
        };

        let feature_background_color = match tile.background_color {
            Some(color) => color,
            None => Color::NONE,
        };

        let feature_atlas = tile.atlas.clone();

        render_context.set_atlas(MapLayer::Features, index, feature_atlas);
        render_context.set_index(MapLayer::Features, index, feature_index);
        render_context.set_foreground_color(MapLayer::Features, index, feature_color);
        render_context.set_background_color(MapLayer::Features, index, feature_background_color);
    } else {
        error!("Theme is missing FeatureType::{}", u8::from(feature_type));
    }

    // ITEMS

    let item_type = match map.item_types.get(index) {
        Some(item_types) => *item_types.first().unwrap_or(&ItemType::None),
        None => ItemType::None,
    };

    if let Some(tile) = item_theme.get_tile_definition(item_type.into()) {
        let item_index = tile.index;
        let item_color = match tile.foreground_color {
            Some(color) => color,
            None => Color::WHITE,
        };

        let item_background_color = match tile.background_color {
            Some(color) => color,
            None => Color::NONE,
        };

        let item_atlas = tile.atlas.clone();

        render_context.set_atlas(MapLayer::Items, index, item_atlas);
        render_context.set_index(MapLayer::Items, index, item_index);
        render_context.set_foreground_color(MapLayer::Items, index, item_color);
        render_context.set_background_color(MapLayer::Items, index, item_background_color);
    } else {
        error!("Theme is missing itemType::{}", u8::from(item_type));
    }
}
