use crate::prelude::*;

pub struct DebugEguiPlugin;
impl Plugin for DebugEguiPlugin {
    fn build(&self, app: &mut App,) {
        // Inspector Egui
        app.add_plugin(DefaultInspectorConfigPlugin,).add_plugin(EguiPlugin,);

        // Systems
        app.insert_resource(DebugUIState::default(),)
            .add_system_to_stage(CoreStage::PreUpdate, show_ui_system.at_end(),)
            .add_system(set_camera_viewport,)
            .add_system(user_input,)
            .add_system(set_debug_title,);
    }
}
