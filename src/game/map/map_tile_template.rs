use crate::prelude::*;
use std::fs::{read_dir, read_to_string};

const MAP_TILE_TEMPLATE_PATH: &str = "assets/data/map_tiles/";

pub struct MapTileTemplates {
    pub templates: HashMap<u64, Vec<MapTileTemplate>>,
}

impl Default for MapTileTemplates {
    fn default() -> Self {
        let mut templates = HashMap::new();

        match read_dir(MAP_TILE_TEMPLATE_PATH) {
            Ok(paths) => {
                paths.filter_map(|x| x.ok()).for_each(|dir| {
                    let path = dir.path();
                    if let Ok(contents) = read_to_string(path) {
                        if let Ok(template) = ron::from_str::<MapTileTemplate>(&contents) {
                            if !templates.contains_key(&template.tile_type) {
                                templates.insert(template.tile_type, Vec::new());
                            }
                            if let Some((_, vec)) = templates.get_key_value_mut(&template.tile_type)
                            {
                                vec.push(template);
                            }
                        }
                    }
                });

                Self { templates }
            }
            Err(e) => {
                println!("{}", e);
                Self { templates: HashMap::new() }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MapTileTemplate {
    pub tile_type: u64,
    pub image_index: usize,
    pub image_color: Color,
    pub image_color_background: Color,
    pub required_movement: Vec<MovementType>,
    pub required_vision_to_see: Vec<VisionType>,
    pub required_vision_to_see_through: Vec<VisionType>,
}

// #[allow(dead_code)]
// impl MapTileTemplate {
//     pub fn spawn(&self, index: impl Point2d, map: &mut Map) { map.set_from_template(index,
// self); } }
