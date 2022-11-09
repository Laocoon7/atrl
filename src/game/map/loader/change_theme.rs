use crate::game::prelude::*;

pub struct ChangeTheme {
    pub terrain_theme_name: Option<String>,
    pub feature_theme_name: Option<String>,
    pub item_theme_name: Option<String>,
}

impl Default for ChangeTheme {
    fn default() -> Self {
        Self {
            terrain_theme_name: Some(DEFAULT_TERRAIN_THEME_NAME.to_string()),
            feature_theme_name: Some(DEFAULT_FEATURE_THEME_NAME.to_string()),
            item_theme_name: Some(DEFAULT_ITEM_THEME_NAME.to_string()),
        }
    }
}
