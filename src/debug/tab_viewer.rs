use crate::prelude::*;
use bevy_inspector_egui::bevy_inspector::{
    hierarchy::hierarchy_ui, ui_for_all_assets, ui_for_entity, ui_for_resources,
};
use bevy_inspector_egui::egui;

pub struct TabViewer<'a,> {
    pub world: &'a mut World,
    pub viewport_rect: &'a mut egui::Rect,
    pub selected_entities: &'a mut SelectedEntities,
}

impl egui_dock::TabViewer for TabViewer<'_,> {
    type Tab = DebugWindow;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab,) {
        match tab {
            DebugWindow::GameView => {
                (*self.viewport_rect, _,) =
                    ui.allocate_exact_size(ui.max_rect().size(), egui::Sense::hover(),);
            }
            DebugWindow::Hierarchy => {
                ui.label("Press `1` to hide",);
                ui.add_space(10.0,);

                hierarchy_ui(self.world, ui, self.selected_entities,);
            }
            DebugWindow::Resources => {
                ui.label("Press `2` to hide",);
                ui.add_space(10.0,);

                ui_for_resources(self.world, ui,);
            }
            DebugWindow::Assets => {
                ui.label("Press `2` to hide",);
                ui.add_space(10.0,);

                ui_for_all_assets(self.world, ui,);
            }
            DebugWindow::Inspector => {
                ui.label("Press `3` to hide",);
                ui.add_space(10.0,);

                for entity in self.selected_entities.iter() {
                    ui_for_entity(self.world, entity, ui, self.selected_entities.len() > 1,);
                }
            }
        }
    }

    fn title(&mut self, window: &mut Self::Tab,) -> egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab,) -> bool {
        !matches!(window, DebugWindow::GameView)
    }
}
