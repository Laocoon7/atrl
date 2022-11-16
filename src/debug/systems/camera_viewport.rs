use crate::prelude::*;
use bevy::render::camera::Viewport;
use bevy_inspector_egui::bevy_egui;

// make camera only render to view not obstructed by UI
pub fn set_camera_viewport(
    cams: Cameras,
    windows: Res<Windows>,
    my_app: Res<DebugUIState>,
    mut cameras: Query<&mut Camera, Without<UICamera>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
) {
    let cam_entity = cams.get_camera_entity(0).unwrap();
    let mut cam = cameras.get_mut(cam_entity).unwrap();

    if !my_app.window_visibility.overall {
        cam.viewport = None;
    } else {
        let window = windows.primary();
        let scale_factor = window.scale_factor() * egui_settings.scale_factor;

        let viewport_pos = my_app.viewport_rect.left_top().to_vec2() * scale_factor as f32;
        let viewport_size = my_app.viewport_rect.size() * scale_factor as f32;

        cam.viewport = Some(Viewport {
            depth: 0.0..1.0,
            physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
            physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
        });
    }
}
