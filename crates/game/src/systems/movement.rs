use crate::prelude::*;

pub fn movement(
    mut q_transform: Query<&mut Transform>,
    q_local_position: Query<(Entity, &LocalPosition), Changed<LocalPosition>>,
) {
    for (entity, local_position) in q_local_position.iter() {
        if let Ok(mut transform) = q_transform.get_mut(entity) {
            transform.translation = Vec3::new(
                local_position.0.x as f32 + 0.5,
                local_position.0.y as f32 + 0.5,
                transform.translation.z,
            );
        }
    }
}
