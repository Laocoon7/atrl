use crate::prelude::*;

pub fn update_targeting(
    tilesets: Tilesets,
    mut commands: Commands,
    mut target_q: Query<(&Position, &AIComponent, &mut TargetVisualizer), Changed<AIComponent>>,
) {
    for (ai_position, ai_component, mut target_visualizer) in target_q.iter_mut() {
        if ai_component.preferred_action.is_some() {
            if let Some(ActionType::Movement(pos) | ActionType::Attack(pos)) = ai_component.preferred_action {
                target_visualizer.update(
                    &mut commands,
                    &tilesets,
                    ai_position.gridpoint(),
                    pos.gridpoint(),
                );
            } else {
                target_visualizer.clear(&mut commands);
                continue;
            };
        }
    }
}
