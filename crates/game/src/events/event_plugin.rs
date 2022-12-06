use crate::prelude::*;

// Handles the ProccessEvents Stage
#[derive(Copy, Clone)]
pub struct ProccessEventsPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> ProccessEventsPlugin<T> {
    fn setup_events(self, app: &mut App) -> Self {
        app.init_resource::<Events<OnMapLoaded>>()
            .init_resource::<Events<OnMapTileEnter>>()
            .init_resource::<Events<OnMapTileExit>>();

        self
    }

    fn setup_event_stages(self, app: &mut App) -> Self {
        use CoreStage::*;
        app.add_stage_after(
            AtrlStage::ProcessTurns,
            AtrlStage::ProccessEvents,
            SystemStage::parallel(),
        )
        .add_stage_before(Last, AtrlStage::CleanupEvents, SystemStage::parallel());
        self
    }
}

impl<T: StateNext> Plugin for ProccessEventsPlugin<T> {
    fn build(&self, app: &mut App) {
        self.setup_event_stages(app).setup_events(app);

        // Bevy Events
        app.add_system_set_to_stage(
            AtrlStage::CleanupEvents,
            ConditionSet::new()
                .run_in_state(self.state_running)
                .with_system(event_cleaner::<OnMapLoaded>)
                .with_system(event_cleaner::<OnMapTileEnter>)
                .with_system(event_cleaner::<OnMapTileExit>)
                .into(),
        );

        // Game Events
        app.add_system_set_to_stage(
            AtrlStage::ProccessEvents,
            ConditionSet::new().run_in_state(self.state_running).with_system(update_transforms).into(),
        );
    }
}
