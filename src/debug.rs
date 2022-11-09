use crate::prelude::*;
use bevy::diagnostic::{Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

const DEBUG_UI_STAGE: &str = "debug_ui_stage";

/// This system will then change the title during execution
fn set_debug_title(
    mut windows: ResMut<Windows>,
    diagnostics: Res<Diagnostics>,
    state: Res<CurrentGameState>,
) {
    if let Some(window) = windows.get_primary_mut() {
        let title = format!(
            "Avg. FPS: {:.02} | Entity Count: {} | CurrentState: {:?}",
            diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).unwrap().average().unwrap_or_default(),
            diagnostics
                .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                .unwrap()
                .value()
                .unwrap_or_default(),
            state.0
        );

        window.set_title(title);
    }
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.insert_resource(bevy::log::LogSettings {
                level: bevy::log::Level::INFO,
                filter: "gfx_backend_metal=warn,wgpu_core=warn, bevy_render=info,lain=debug,bevy_render::render_resource::pipeline_cache=debug".to_string(),
            });

            // Fps / Entity Tracking
            app.add_plugin(FrameTimeDiagnosticsPlugin).add_plugin(EntityCountDiagnosticsPlugin);

            // Inspector Egui
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Equipable>()
                .register_inspectable::<Health>()
                .register_inspectable::<MovementType>()
                .register_inspectable::<LocalPosition>()
                .register_inspectable::<WorldPosition>()
                .register_inspectable::<VisionType>();

            app.add_stage_after(
                CoreStage::PostUpdate,
                DEBUG_UI_STAGE,
                SystemStage::parallel()
                    .with_system_set(SystemSet::new().with_system(set_debug_title)),
            );
        } else {
            app.insert_resource(bevy::log::LogSettings {
                level: bevy::log::Level::WARN,
                ..Default::default()
            });
        }
    }
}

#[cfg(feature = "debug")]
pub mod colorized {
    use crate::prelude::{Map, TerrainType};
    use colored::Colorize;

    pub trait Colorized {
        fn to_colorized_string(&self) -> String;
    }

    impl Colorized for TerrainType {
        fn to_colorized_string(&self) -> String {
            format!(
                "{}",
                match self {
                    Self::None => "X".bright_yellow(),
                    Self::Wall => "#".bright_yellow(),
                    Self::Floor => ".".bright_green(),
                    Self::Water => "~".bright_blue(),
                }
            )
        }
    }

    impl Colorized for Map {
        fn to_colorized_string(&self) -> String {
            let mut buffer = format!("\nMap (w: {}, h: {})\n", self.size.x, self.size.y);
            let line: String = (0..(self.size.x + 2)).into_iter().map(|_| '-').collect();
            buffer = format!("{}{}\n", buffer, line);

            for y in 0..self.size.y as usize {
                buffer = format!("{}|", buffer);
                for tile in self.terrain_types.iter_column_unchecked(y) {
                    buffer = format!("{}{}", buffer, tile.to_colorized_string());
                }
                buffer = format!("{}|\n", buffer);
            }

            format!("{}{}", buffer, line)
        }
    }
}
