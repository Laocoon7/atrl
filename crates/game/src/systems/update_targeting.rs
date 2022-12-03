use crate::prelude::*;

pub fn update_targeting(mut target_q: Query<&mut TargetVisualizer, Changed<AIComponent>>) {
    for mut target_visualizer in target_q.iter_mut() {
        println!("Updating targeting");
        // target_visualizer.update();
    }
}
