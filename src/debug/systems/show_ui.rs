use crate::prelude::*;
use bevy_inspector_egui::bevy_egui;

pub fn show_ui_system(world: &mut World) {
    let mut egui_context = world.resource_mut::<bevy_egui::EguiContext>().ctx_mut().clone();
    world.resource_scope::<DebugUIState, _>(|world, mut ui_state| {
        ui_state.ui(world, &mut egui_context)
    });
}
