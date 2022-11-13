use crate::game::prelude::internal::*;
use crate::prelude::*;

const RON_EXT: &str = "ron";
const TERRAIN_DIR: &str = "assets/tilesets/terrain/";
const FEATURE_DIR: &str = "assets/tilesets/features/";
const ITEM_DIR: &str = "assets/tilesets/items/";

#[derive(Resource)]
pub struct TileLoader {
    pub terrain_themes: HashMap<String, Theme>,
    pub feature_themes: HashMap<String, Theme>,
    pub item_themes: HashMap<String, Theme>,
}

impl TileLoader {
    pub fn get_terrain_theme(&self, theme_name: &str) -> Option<&Theme> {
        if self.terrain_themes.contains_key(theme_name) {
            Some(&self.terrain_themes[theme_name])
        } else {
            None
        }
    }

    pub fn get_terrain_theme_or_default(&self, theme_name: &str) -> &Theme {
        self.get_terrain_theme(theme_name).map_or_else(
            || self.get_default_theme_terrain().expect("No default theme found!!!"),
            |theme| theme,
        )
    }

    fn get_default_theme_terrain(&self) -> Option<&Theme> { self.terrain_themes.values().next() }

    pub fn get_feature_theme(&self, theme_name: &str) -> Option<&Theme> {
        if self.feature_themes.contains_key(theme_name) {
            Some(&self.feature_themes[theme_name])
        } else {
            None
        }
    }

    pub fn get_feature_theme_or_default(&self, theme_name: &str) -> &Theme {
        self.get_feature_theme(theme_name).map_or_else(
            || self.get_default_theme_feature().expect("No default theme found!!!"),
            |theme| theme,
        )
    }

    fn get_default_theme_feature(&self) -> Option<&Theme> { self.feature_themes.values().next() }

    pub fn get_item_theme(&self, theme_name: &str) -> Option<&Theme> {
        if self.terrain_themes.contains_key(theme_name) {
            Some(&self.terrain_themes[theme_name])
        } else {
            None
        }
    }

    pub fn get_item_theme_or_default(&self, theme_name: &str) -> &Theme {
        self.get_item_theme(theme_name).map_or_else(
            || self.get_default_theme_item().expect("No default theme found!!!"),
            |theme| theme,
        )
    }

    fn get_default_theme_item(&self) -> Option<&Theme> { self.item_themes.values().next() }
}

impl FromWorld for TileLoader {
    fn from_world(world: &mut World) -> Self {
        let mut system_state: SystemState<(Res<AssetServer>, ResMut<Assets<TextureAtlas>>)> =
            SystemState::new(world);
        let (asset_server, mut atlases) = system_state.get_mut(world);

        let terrain_themes = load_themes(TERRAIN_DIR, &asset_server, &mut atlases);
        let feature_themes = load_themes(FEATURE_DIR, &asset_server, &mut atlases);
        let item_themes = load_themes(ITEM_DIR, &asset_server, &mut atlases);

        info!("TerrainThemes Loaded: {}", terrain_themes.len());
        info!("FeatureThemes Loaded: {}", feature_themes.len());
        info!("ItemThemes Loaded: {}", item_themes.len());

        Self { terrain_themes, feature_themes, item_themes }
    }
}

fn load_themes(
    path: &str,
    asset_server: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> HashMap<String, Theme> {
    let mut texture_atlases = HashMap::new();
    let mut default_name_count = 0;

    match std::fs::read_dir(path) {
        Ok(paths) => paths.filter_map(|x| x.ok()).for_each(|dir| {
            let path = dir.path();

            if path.extension().map(|ext| ext == RON_EXT).is_some() {
                let path_name = match path.file_name() {
                    Some(n) => n.to_str().map_or_else(
                        || {
                            let n = format!("Default{}", default_name_count);
                            default_name_count += 1;
                            n
                        },
                        |n| n.to_string(),
                    ),
                    None => {
                        let n = format!("Default{}", default_name_count);
                        default_name_count += 1;
                        n
                    }
                };

                match std::fs::read_to_string(path) {
                    Ok(contents) => match ron::from_str::<TextureAtlasTemplate>(&contents) {
                        Ok(template) => {
                            texture_atlases.insert(path_name, template);
                        }
                        Err(e) => error!("{}", e),
                    },
                    Err(e) => error!("{}", e),
                }
            }
        }),
        Err(e) => error!("{}", e),
    }

    let mut themes = HashMap::new();
    for (theme_name, texture_atlas) in texture_atlases.iter() {
        // get image_handle
        let image_handle = asset_server.load(&texture_atlas.file);

        // load texture_atlas if necessary
        // Find and get Handle<TextureAtlas> if exists
        let mut atlas_handle = None;
        for (id, atlas) in atlases.iter() {
            if atlas.texture == image_handle {
                let tmp: Handle<TextureAtlas> = Handle::weak(id);
                atlas_handle = Some(tmp);
                break;
            }
        }

        // Build / Load new TextureAtlas if we didn't find one
        if atlas_handle.is_none() {
            let padding = Vec2::new(
                texture_atlas.padding_x.unwrap_or(0.0),
                texture_atlas.padding_y.unwrap_or(0.0),
            );

            let offset = Vec2::new(
                texture_atlas.offset_x.unwrap_or(0.0),
                texture_atlas.offset_y.unwrap_or(0.0),
            );

            let texture_atlas_handle = atlases.add(TextureAtlas::from_grid(
                image_handle,
                Vec2::new(texture_atlas.tile_width, texture_atlas.tile_height),
                texture_atlas.columns,
                texture_atlas.rows,
                Some(padding),
                Some(offset),
            ));

            atlas_handle = Some(texture_atlas_handle);
        }

        atlas_handle.map_or((), |handle| {
            let mut theme = Theme { tiles: HashMap::new() };
            for template in texture_atlas.tile_templates.iter() {
                let foreground_color = template.foreground_color.as_ref().map(|color_definition| {
                    Color::rgba_u8(
                        color_definition.r,
                        color_definition.g,
                        color_definition.b,
                        color_definition.a,
                    )
                });

                let background_color = template.background_color.as_ref().map(|color_definition| {
                    Color::rgba_u8(
                        color_definition.r,
                        color_definition.g,
                        color_definition.b,
                        color_definition.a,
                    )
                });

                let tile_definition = TileDefinition {
                    index: (template.x, template.y).as_index(texture_atlas.columns),
                    atlas: handle.clone(),
                    foreground_color,
                    background_color,
                };
                theme.tiles.insert(template.tile_type, tile_definition);
            }
            themes.insert(theme_name.clone(), theme);
        })
    }
    themes
}
