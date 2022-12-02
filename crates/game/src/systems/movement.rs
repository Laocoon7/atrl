use crate::prelude::*;

pub fn movement(mut q_position: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (position, mut transform) in q_position.iter_mut() {
        transform.translation = position.translation();
    }
}
