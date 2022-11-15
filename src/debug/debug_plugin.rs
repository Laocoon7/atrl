use crate::prelude::*;
use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    log::LogPlugin,
};

const DEBUG_UI_STAGE: &str = "debug_ui_stage";

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "wgpu_hal=warn,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,bevy_render::render_resource::pipeline_cache=warn,bevy_app=debug".to_string(),
        });

        // Fps / Entity Tracking
        app.add_plugin(FrameTimeDiagnosticsPlugin).add_plugin(EntityCountDiagnosticsPlugin);

        // Inspector Egui
        app.add_plugin(DefaultInspectorConfigPlugin).add_plugin(EguiPlugin);

        // // Systems
        app.add_system(inspector_ui).add_system(set_debug_title);

        // app.add_stage_after(
        //     CoreStage::PostUpdate,
        //     DEBUG_UI_STAGE,
        //     SystemStage::parallel().with_system_set(
        //         ConditionSet::new()
        //             // .with_system(set_debug_title)
        //             .with_system(inspector_ui)
        //             .into(),
        //     ),
        // );
    }
}
