use crate::prelude::*;

#[derive(SystemLabel, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum MapSystem {
    Update,
    Draw,
}

pub struct MapPlugin<T> {
    pub state_construct: T,
    pub state_running: T,
}

impl<T: StateNext> Plugin for MapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<ThemeServer>()
            .add_enter_system(self.state_construct.clone(), load_themes_into_theme_server)
            .add_system_set_to_stage(
                CoreStage::Last,
                ConditionSet::new()
                    .run_in_state(self.state_running.clone())
                    .label(MapSystem::Update)
                    .before(MapSystem::Draw)
                    .with_system(update_animations)
                    .into(),
            )
            .add_system_set_to_stage(
                CoreStage::Last,
                ConditionSet::new()
                    .run_in_state(self.state_running.clone())
                    .label(MapSystem::Draw)
                    .with_system(redraw_map_renderers)
                    .into(),
            );
    }
}
