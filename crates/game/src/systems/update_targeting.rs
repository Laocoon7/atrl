use crate::prelude::*;

pub fn update_targeting(
    tilesets: Tilesets,
    mut commands: Commands,
    mut target_q: Query<(&Position, &AIComponent, &mut TargetVisualizer), Changed<AIComponent>>,
) {
    for (ai_position, ai_component, mut target_visualizer) in target_q.iter_mut() {
        let destination = match &ai_component.preferred_action {
            Some(action) => match action {
                ActionType::Wait => continue,
                ActionType::Attack(_) => continue,
                ActionType::MovementDelta(_) => continue,
                ActionType::Movement(to_pos) => to_pos,
            },
            None => continue,
        };

        target_visualizer.update(
            &mut commands,
            &tilesets,
            ai_position.gridpoint(),
            destination.gridpoint(),
        );
    }
}
