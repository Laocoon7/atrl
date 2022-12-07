use crate::prelude::*;

pub fn update_mouse_position(
    cameras: Cameras,
    wnds: Res<Windows>,
    map_manager: MapManager,
    mut mouse_position: ResMut<MousePosition>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let Some(camera_entity) = cameras.get_camera_entity(CameraId::Map)else{return};
    let Ok((camera, camera_transform)) = q_camera.get(camera_entity)else{return};

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width(), wnd.height());

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        mouse_position.set_mouse_position(Position::new(
            map_manager.get_current_world_position(),
            LocalPosition::new(world_pos.x as u32, world_pos.y as u32, MapLayer::UI as u32),
        ));

        println!("Mouse position: {}", *mouse_position);
    }
}
