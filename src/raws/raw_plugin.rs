use crate::prelude::*;
use crate::raws::prelude::internal::*;

pub struct RawPlugin<T> {
    pub state_asset_load: T,
    pub state_asset_load_failure: T,
}

impl<T: StateNext + std::default::Default> Plugin for RawPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(ProgressPlugin::new(self.state_asset_load.clone()));

        app.add_plugin(SplashPlugin {
            state_asset_load: self.state_asset_load.clone(),
            ..Default::default()
        })
        .add_loading_state(
            LoadingState::new(self.state_asset_load.clone())
                .with_collection::<TextureAssets>()
                .with_collection::<FontAssets>()
                .on_failure_continue_to_state(self.state_asset_load_failure.clone()),
        )
        .add_system(check_progress.run_in_state(self.state_asset_load.clone()));
    }
}
