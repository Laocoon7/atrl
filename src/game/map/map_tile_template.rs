use crate::game::prelude::*;

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
    pub required_vision_to_see: Vec<VisionType>,
    pub required_vision_to_see_through: Vec<VisionType>,
    pub required_movement: Vec<MovementType>,
}

impl MapTileTemplate {
    pub fn spawn(&self, commands: &mut Commands) -> Entity {
        commands
            .spawn()
            .insert(MapTile {
                tile_type: self.tile_type,
                required_vision_to_see: self.required_vision_to_see.clone(),
                required_vision_to_see_through: self.required_vision_to_see_through.clone(),
                required_movement: self.required_movement.clone(),
            })
            .id()
    }
}

#[derive(Inspectable, Component, Clone, PartialEq, Debug)]
pub struct MapTile {
    tile_type: u64,
    required_vision_to_see: Vec<VisionType>,
    required_vision_to_see_through: Vec<VisionType>,
    required_movement: Vec<MovementType>,
}

#[derive(Inspectable, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VisionType {
    Blind = 0,
    BlackAndWhite,
    Colored,
    Infared,
    XRay,
}

impl Default for VisionType {
    fn default() -> Self { VisionType::Blind }
}

#[derive(Inspectable, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum MovementType {
    None = 0,
    Walk,
    Run,
    Swim,
    Fly,
    Phase,
}

impl Default for MovementType {
    fn default() -> Self { MovementType::None }
}
