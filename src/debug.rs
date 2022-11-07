use crate::game::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<VisionType>()
            .register_inspectable::<MovementType>();
        //.register_inspectable::<TYPE>()
    }
}
