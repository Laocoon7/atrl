use crate::prelude::*;

pub fn update_targeting(
    tilesets: Tilesets,
    mut commands: Commands,
    map_manager: MapManager,
    mut target_q: Query<(&Position, &AIComponent, &mut TargetVisualizer), Changed<AIComponent>>,
) {
    for (ai_position, ai_component, mut target_visualizer) in target_q.iter_mut() {
        if let Some(ActionType::Movement(destination) | ActionType::Attack(destination)) =
            ai_component.preferred_action
        {
            target_visualizer.update(
                &mut commands,
                &map_manager,
                &tilesets,
                *ai_position,
                destination,
            );
        } else {
            target_visualizer.clear(&mut commands);
            continue;
        };
    }
}
