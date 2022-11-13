use crate::prelude::*;

pub fn redraw_map_renderers(
    mut commands: Commands,
    mut q_map_renderer: Query<&mut MapRenderer>,
    theme_server: Res<ThemeServer>,
    white_pixel: Res<WhitePixel>,
    mut q_foreground_tiles: Query<
        (&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &mut ForegroundTile),
        Without<AnimatedTile>,
    >,
    mut q_animated_tiles: Query<
        (&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &mut AnimatedTile),
        Without<ForegroundTile>,
    >,
    mut q_background_tiles: Query<&mut Sprite, With<BackgroundTile>>,
) {
    let mut selected_theme = None;

    for mut map_renderer in q_map_renderer.iter_mut() {
        let mut add_entities = Vec::new();
        let mut remove_indexes = Vec::new();
        for actions in &map_renderer.actions {
            for action in actions {
                match action {
                    RenderAction::SetTheme(theme_name) => {
                        if let Some(theme) = theme_server.get_theme(theme_name) {
                            selected_theme = Some(theme);
                        }
                    }
                    RenderAction::SetTile(index, tile_type) => {
                        if let Some(theme) = selected_theme {
                            let layer = MapRenderer::layer_id_to_foreground_layer(theme.theme_type);

                            let mut background_color = Color::NONE;

                            // Foreground Tile
                            let mut found_tile = None;
                            for tile in &theme.tiles {
                                if tile.tile_type == *tile_type {
                                    found_tile = Some(tile);
                                    break;
                                }
                            }

                            if let Some(tile) = found_tile {
                                if let Some(foreground_tile) = ForegroundTile::from_tile(
                                    tile,
                                    &theme_server,
                                    map_renderer.size.width() as usize,
                                ) {
                                    background_color = foreground_tile.bg_color;
                                    match map_renderer.get_entity(layer, *index) {
                                        Some(entity) => {
                                            // Get entity from foreground_tiles
                                            if let Ok((mut handle, mut sprite, mut old_tile)) =
                                                q_foreground_tiles.get_mut(entity)
                                            {
                                                *handle = foreground_tile.texture_atlas.clone();
                                                sprite.index = foreground_tile.index;
                                                *old_tile = foreground_tile;
                                            } else {
                                                error!("ForegroundTile: Entity missing components");
                                            }
                                        }
                                        None => {
                                            // spawn entity
                                            let entity = commands
                                                .spawn(ForegroundTileBundle::from_foreground_tile(
                                                    foreground_tile,
                                                    index.as_vec2().extend(layer as f32),
                                                ))
                                                .id();
                                            add_entities.push((layer, *index, Some(entity)));
                                        }
                                    }
                                }
                            } else {
                                for animation in &theme.animations {
                                    if animation.tile_type == *tile_type {
                                        if let Some((animated_tile, first_tile)) =
                                            AnimatedTile::from_animation(
                                                animation,
                                                &theme_server,
                                                map_renderer.size.width() as usize,
                                            )
                                        {
                                            background_color = first_tile.bg_color;
                                            match map_renderer.get_entity(layer, *index) {
                                                Some(entity) => {
                                                    // get entity from animated_tiles
                                                    if let Ok((
                                                        mut handle,
                                                        mut sprite,
                                                        mut old_tile,
                                                    )) = q_animated_tiles.get_mut(entity)
                                                    {
                                                        *handle = first_tile.texture_atlas.clone();
                                                        sprite.index = first_tile.index;
                                                        *old_tile = animated_tile;
                                                    } else {
                                                        error!("AnimatedTile: Entity missing components");
                                                    }
                                                }
                                                None => {
                                                    // spawn entity
                                                    if let Some(bundle) =
                                                        AnimatedTileBundle::from_animation(
                                                            animated_tile,
                                                            index.as_vec2().extend(layer as f32),
                                                        )
                                                    {
                                                        let entity = commands.spawn(bundle).id();
                                                        add_entities.push((
                                                            layer,
                                                            *index,
                                                            Some(entity),
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Background Tile
                            match map_renderer.get_entity(layer - 1, *index) {
                                Some(entity) => {
                                    if let Ok(mut sprite) = q_background_tiles.get_mut(entity) {
                                        sprite.color = background_color;
                                    } else {
                                        error!("BackgroundTile: Entity missing components")
                                    }
                                }
                                None => {
                                    if background_color.a() > 0.0 {
                                        let entity = commands
                                            .spawn(BackgroundTileBundle::from_color(
                                                &white_pixel.0,
                                                background_color,
                                                index.as_vec2().extend((layer - 1) as f32),
                                            ))
                                            .id();
                                        add_entities.push((layer - 1, *index, Some(entity)));
                                    }
                                }
                            }
                        }
                    }
                    RenderAction::ClearTile(index) => {
                        remove_indexes.push(*index);
                        for entity in map_renderer.get_entities(*index) {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                }
            }
        }
        map_renderer.clear_actions();

        for index in remove_indexes {
            map_renderer.clear_entities(index);
        }

        for (layer, index, entity) in add_entities {
            map_renderer.set_entity(layer, index, entity);
        }
    }
}
