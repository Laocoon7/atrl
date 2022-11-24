use crate::prelude::*;
use bevy_inspector_egui::egui;
use egui_dock::NodeIndex;

#[derive(Debug, Eq, Hash, PartialEq,)]
pub enum DebugWindow {
    GameView,
    Hierarchy,
    Resources,
    Assets,
    Inspector,
}

#[derive(Debug, Eq, Hash, PartialEq,)]
pub struct WindowVisibility {
    pub assets: bool,
    pub overall: bool,
    pub hierarchy: bool,
    pub resources: bool,
    pub inspector: bool,
}

impl Default for WindowVisibility {
    fn default() -> Self {
        Self { assets: true, overall: true, hierarchy: true, resources: true, inspector: true, }
    }
}

#[derive(Resource,)]
pub struct DebugUIState {
    pub window_visibility: WindowVisibility,
    pub viewport_rect: egui::Rect,
    selected_entities: SelectedEntities,
    pub tree: egui_dock::Tree<DebugWindow,>,
}

impl Default for DebugUIState {
    fn default() -> Self {
        let mut tree = egui_dock::Tree::new(vec![DebugWindow::GameView],);
        let [game, _inspector] =
            tree.split_right(NodeIndex::root(), 0.75, vec![DebugWindow::Inspector],);
        let [game, _hierarchy] = tree.split_left(game, 0.2, vec![DebugWindow::Hierarchy],);
        let [_game, _bottom] =
            tree.split_below(game, 0.8, vec![DebugWindow::Resources, DebugWindow::Assets],);

        Self {
            tree,
            viewport_rect: egui::Rect::NOTHING,
            window_visibility: WindowVisibility::default(),
            selected_entities: SelectedEntities::default(),
        }
    }
}

impl DebugUIState {
    pub fn update_ui(&mut self,) {
        let mut tree = egui_dock::Tree::new(vec![DebugWindow::GameView],);

        let mut game_node = if self.window_visibility.inspector {
            let [game, _inspector] =
                tree.split_right(NodeIndex::root(), 0.75, vec![DebugWindow::Inspector],);
            game
        } else {
            NodeIndex(0,)
        };

        if self.window_visibility.hierarchy {
            let [game, _hierarchy] = tree.split_left(game_node, 0.2, vec![DebugWindow::Hierarchy],);
            game_node = game;
        }

        if self.window_visibility.assets {
            let [_game, _bottom] = tree.split_below(
                game_node,
                0.8,
                vec![DebugWindow::Resources, DebugWindow::Assets],
            );
        }

        self.tree = tree;
    }

    pub fn ui(&mut self, world: &mut World, ctx: &mut egui::Context,) {
        if self.window_visibility.overall {
            let mut tab_viewer = TabViewer {
                world,
                viewport_rect: &mut self.viewport_rect,
                selected_entities: &mut self.selected_entities,
            };
            egui_dock::DockArea::new(&mut self.tree,).show(ctx, &mut tab_viewer,);
        }
    }
}
