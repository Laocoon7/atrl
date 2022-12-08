use crate::prelude::*;

pub fn update_targeting(
    tilesets: Tilesets,
    mut commands: Commands,
    map_manager: MapManager,
    mut target_q: Query<(&Position, &mut AIComponent, &mut TargetVisualizer), Changed<AIComponent>>,
) {
    for (ai_position, mut ai_component, mut target_visualizer) in target_q.iter_mut() {
        // let Some(action) = &ai_component.preferred_action {}else{}

        let Some(action) = ai_component.get_action() else{
            target_visualizer.clear(&mut commands);
            continue;
        };

        if let Some(target) = action.get_target_position() {
            target_visualizer.update(&mut commands, &map_manager, &tilesets, *ai_position, target);
        } else {
            target_visualizer.clear(&mut commands);
            continue;
        }
    }
}
