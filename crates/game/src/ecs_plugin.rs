use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum AtrlStage {
    ProccessEvents,
    CleanupEvents,
}

#[derive(Copy, Clone)]
pub struct EcsPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> EcsPlugin<T> {
    pub fn setup_stages(self, app: &mut App) -> Self {
        use CoreStage::*;

        // CoreStage::First
        //   (after) => BigBrainStage::Scorers
        //      (after) => BigBrainStage::Thinkers
        // CoreStage::PreUpdate
        //   (after) => BigBrainStage::Actions
        //      (after) => AtrlStage::ProccessEvents (movement_sys / perform_turns_sys)
        // CoreStage::Update
        // CoreStage::PostUpdate
        // CoreStage::Last => (cull_dead_sys => fov_sys)
        //   (before) => AtrlStage::CleanupEvents
        //   (after) => BigBrainStage::Cleanup
        app.add_stage_after(
            BigBrainStage::Actions,
            AtrlStage::ProccessEvents,
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

impl<T: StateNext> Plugin for EcsPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            // Player
            .add_plugin(PlayerPlugin {
                state_running: self.state_running,
            })
            // AI
            .add_plugin(AIPlugin {
                state_running: self.state_running,
            });

        self.setup_stages(app).setup_events(app);

        // Startup
        app.add_enter_system_set(
            self.state_running,
            ConditionSet::new().with_system(fov).into(),
        );

        // Process Events
        app.add_system_set_to_stage(
            AtrlStage::ProccessEvents,
            ConditionSet::new().run_in_state(self.state_running).with_system(perform_turns).into(),
        )
        .add_system_set_to_stage(
            AtrlStage::ProccessEvents,
            ConditionSet::new().run_in_state(self.state_running).with_system(movement).into(),
        );

        app.add_system_set_to_stage(
            CoreStage::Last,
            ConditionSet::new()
                .run_in_state(self.state_running)
                .label("cull_dead")
                .with_system(cull_dead)
                .into(),
        )
        .add_system_set_to_stage(
            CoreStage::Last,
            ConditionSet::new().run_in_state(self.state_running).after("cull_dead").with_system(fov).into(),
        );
    }
}
