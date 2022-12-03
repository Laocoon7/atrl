use crate::prelude::*;

pub fn update_targeting(
    tilesets: Tilesets,
    mut commands: Commands,
    mut target_q: Query<(&Position, &AIComponent, &mut TargetVisualizer), Changed<AIComponent>>,
) {
    for (ai_position, ai_component, mut target_visualizer) in target_q.iter_mut() {
        let destination = if let Some(ActionType::Movement(to_pos)) = ai_component.preferred_action {
            to_pos
        } else {
            if target_visualizer.get().is_some() {
                target_visualizer.clear(&mut commands)
            }
            continue;
        };

        target_visualizer.update(
            &mut commands,
            &tilesets,
            ai_position.gridpoint(),
            destination.gridpoint(),
        );
    }
}
