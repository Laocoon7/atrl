use crate::prelude::*;

pub fn redraw_map_renderers(
    mut commands: Commands,
    mut q_map_renderer: Query<&mut MapRenderer>,
    theme_server: Res<ThemeServer>,
    white_pixel: Res<WhitePixel>,
    mut tiles: ParamSet<(
        Query<
            (&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &mut ForegroundTile, &BackgroundEntityHolder),
            Without<AnimatedTile>,
        >,
        Query<
            (&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &mut AnimatedTile, &BackgroundEntityHolder),
            Without<ForegroundTile>,
        >,
        Query<&mut Sprite, With<BackgroundTile>>,
    )>,
) {
    let mut selected_theme = None;

    for mut map_renderer in q_map_renderer.iter_mut() {
        let mut add_entities = Vec::new();
        let mut remove_indexes = Vec::new();
        for action in &map_renderer.actions {
            match action {
                RenderAction::SetTheme(theme_name) => {
                    if let Some(theme) = theme_server.get_theme(theme_name) {
                        selected_theme = Some(theme);
                    }
                },
                RenderAction::SetTile(index, tile_type) => {
                    if let Some(theme) = selected_theme {
                        let layer = MapRenderer::layer_id_to_foreground_layer(theme.theme_type);

                        let mut background_color = Color::NONE;
                        let mut background_entity = None;

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
                                        if let Ok((
                                            mut handle,
                                            mut sprite,
                                            mut old_tile,
                                            background_entity_holder
                                        )) = tiles.p0().get_mut(entity)
                                        {
                                            *handle = foreground_tile.texture_atlas.clone();
                                            sprite.index = foreground_tile.index;
                                            *old_tile = foreground_tile;
                                            background_entity = Some(background_entity_holder.entity.clone());
                                        } else {
                                            error!("ForegroundTile: Entity missing components");
                                        }
                                    }
                                    None => {
                                        // spawn entity
                                        let position = index.as_vec2().extend(layer as f32);
                                        let bg_entity = commands.spawn(BackgroundTileBundle::from_color(&white_pixel.handle, Color::NONE, position)).id();
                                        background_entity = Some(bg_entity.clone());
                                        let entity = commands
                                            .spawn(ForegroundTileBundle::from_foreground_tile(
                                                foreground_tile,
                                                bg_entity,
                                                position,
                                            ))
                                            .id();
                                        add_entities.push((layer, *index, Some(entity)));
                                        add_entities.push((layer - 1, *index, Some(bg_entity)));
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
                                                    background_entity_holder
                                                )) = tiles.p1().get_mut(entity)
                                                {
                                                    *handle = first_tile.texture_atlas.clone();
                                                    sprite.index = first_tile.index;
                                                    *old_tile = animated_tile;
                                                    background_entity = Some(background_entity_holder.entity.clone());
                                                } else {
                                                    error!("AnimatedTile: Entity missing components");
                                                }
                                            }
                                            None => {
                                                // spawn entity
                                                let position = index.as_vec2().extend(layer as f32);
                                                let bg_entity = commands.spawn(BackgroundTileBundle::from_color(&white_pixel.handle, Color::NONE, position)).id();
                                                background_entity = Some(bg_entity.clone());

                                                if let Some(bundle) =
                                                    AnimatedTileBundle::from_animation(
                                                        animated_tile,
                                                        bg_entity,
                                                        position,
                                                    )
                                                {
                                                    let entity = commands.spawn(bundle).id();
                                                    add_entities.push((
                                                        layer,
                                                        *index,
                                                        Some(entity),
                                                    ));
                                                    add_entities.push((layer - 1, *index, Some(bg_entity)));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Background Tile
                        if let Some(entity) = background_entity {
                            if let Ok(mut sprite) = tiles.p2().get_mut(entity) {
                                sprite.color = background_color;
                            }
                        }
                    }
                },
                RenderAction::ClearTile(index) => {
                    remove_indexes.push(*index);
                    for entity in map_renderer.get_entities(*index) {
                        commands.entity(entity).despawn_recursive();
                    }
                },
                RenderAction::SetRaw(foreground_entity, texture_atlas_handle, index, foreground_color, background_entity, background_color) => {
                    if let Ok((
                        mut handle,
                        mut sprite,
                        mut _old_tile,
                        _background_entity
                    )) = tiles.p0().get_mut(foreground_entity.clone()) {
                        *handle = texture_atlas_handle.clone();
                        sprite.index = *index;
                        sprite.color = *foreground_color;
                    } else if let Ok((
                        mut handle,
                        mut sprite,
                        mut _old_tile,
                        _background_entity
                    )) = tiles.p1().get_mut(foreground_entity.clone()) {
                        *handle = texture_atlas_handle.clone();
                        sprite.index = *index;
                        sprite.color = *foreground_color;
                    }

                    if let Ok(mut sprite) = tiles.p2().get_mut(background_entity.clone()) {
                        sprite.color = *background_color;
                    }
                },
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
