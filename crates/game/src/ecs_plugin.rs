use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum AtrlStage {
    Startup,
    ConsumeEvents, // process events
    CleanupEvents, // clear events
}

#[derive(Copy, Clone)]
pub struct EcsPlugin<T, R> {
    pub state_running: T,
    pub turn_state_ticking: R,
}

impl<T: StateNext, R: StateNext + Resource> EcsPlugin<T, R> {
    pub fn setup_stages(self, app: &mut App) -> Self {
        app.add_stage_after(
            CoreStage::Update,
            AtrlStage::ConsumeEvents,
            SystemStage::parallel(),
        )
        .add_stage_before(
            CoreStage::Last,
            AtrlStage::CleanupEvents,
            SystemStage::parallel(),
        );

        self
    }

    pub fn setup_events(self, app: &mut App) -> Self {
        app.init_resource::<Events<OnMapLoaded>>()
            .init_resource::<Events<OnMapTileEnter>>()
            .init_resource::<Events<OnMapTileExit>>();

        app.add_system_set_to_stage(
            AtrlStage::CleanupEvents,
            ConditionSet::new()
                .run_in_state(self.state_running)
                .with_system(event_cleaner::<OnMapLoaded>)
                .with_system(event_cleaner::<OnMapTileEnter>)
                .with_system(event_cleaner::<OnMapTileExit>)
                .into(),
        );

        self
    }
}

impl<T: StateNext, R: StateNext + Resource> Plugin for EcsPlugin<T, R> {
    fn build(&self, app: &mut App) {
        self.setup_stages(app).setup_events(app);

        app 
            // Player
            .add_plugin(PlayerPlugin {
                state_running: self.state_running,
            })
            // AI
            .add_plugin(AIPlugin {
                state_running: self.state_running,
                turn_state_ticking: self.turn_state_ticking,
            });

        // Startup
        app.add_enter_system_set(
            self.state_running,
            ConditionSet::new().with_system(fov).into(),
        );

        app.add_system_set_to_stage(
            CoreStage::First,
            ConditionSet::new().run_in_state(self.state_running).with_system(whos_turn).into(),
        );

        app.add_system_set_to_stage(
            AtrlStage::ConsumeEvents,
            ConditionSet::new()
                .run_in_state(self.state_running)
                .run_if_resource_equals(self.turn_state_ticking)
                .with_system(fov)
                .into(),
        );
    }
}
