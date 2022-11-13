use bevy::utils::HashMap;

use crate::prelude::*;

#[derive(Component)]
pub struct MapRenderer {
    pub(crate) size: UVec2,
    pub(crate) actions: Vec<RenderAction>,
    pub(crate) entities: HashMap<u16, Grid<Option<Entity>>>,
}

impl MapRenderer {
    pub fn build(size: impl Size2d) -> Self {
        Self { size: size.as_uvec2(), entities: HashMap::new(), actions: Vec::new() }
    }

    pub fn add_layer<L: Into<u8>>(&mut self, layer: L) -> &mut Self {
        let layer = Self::layer_id_to_foreground_layer(layer.into());

        // background
        self.entities.insert(layer - 1, Grid::new_copy(self.size, None));

        // foreground
        self.entities.insert(layer, Grid::new_copy(self.size, None));

        self
    }

    pub fn finalize(self, commands: &mut Commands) { commands.spawn(self); }

    pub fn get_context(&mut self) -> MapContext { MapContext::new(self) }

    pub(crate) fn add_actions(&mut self, actions: Vec<RenderAction>) {
        for action in actions {
            self.actions.push(action);
        }
    }

    pub(crate) fn clear_actions(&mut self) { self.actions.clear(); }

    pub(crate) fn get_entity(&self, layer: u16, index: impl Point2d) -> Option<Entity> {
        match self.entities.get(&layer) {
            Some(grid) => match grid.get(index) {
                Some(entity_option) => entity_option.as_ref().copied(),
                None => None,
            },
            None => None,
        }
    }

    pub(crate) fn get_entities(&self, index: impl Point2d) -> Vec<Entity> {
        let mut entities = Vec::new();
        for (_layer, grid) in &self.entities {
            match grid.get(index) {
                Some(entity_option) => {
                    if let Some(entity) = entity_option {
                        entities.push(*entity);
                    }
                }
                None => todo!(),
            }
        }
        entities
    }

    pub(crate) fn set_entity(
        &mut self,
        layer: u16,
        index: impl Point2d,
        entity: Option<Entity>,
    ) -> Option<Entity> {
        if let Some(grid) = self.entities.get_mut(&layer) {
            return grid.set(index, entity).unwrap();
        }
        None
    }

    pub(crate) fn clear_entities(&mut self, index: impl Point2d) {
        for (_layer, grid) in &mut self.entities {
            grid.set(index, None);
        }
    }

    /// Returns the foreground layer as stored in self.entities
    /// ```
    /// IN -> OUT
    /// 0 -> 1
    /// 1 -> 3
    /// 2 -> 5
    /// ```
    pub(crate) fn layer_id_to_foreground_layer(layer: u8) -> u16 { (layer as u16 + 1) * 2 - 1 }
}
