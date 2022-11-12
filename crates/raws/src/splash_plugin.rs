use crate::prelude::*;

#[derive(SmartDefault)]
pub struct SplashPlugin<T: std::default::Default> {
    #[default(0.5)]
    pub loading_time: f32,
    pub state_asset_load: T,
}

impl<T: StateNext + std::default::Default> Plugin for SplashPlugin<T> {
    fn build(&self, app: &mut App) {
        let loading_time = self.loading_time;

        app.add_enter_system(
            self.state_asset_load.clone(),
            move |commands: Commands, asset_server: Res<AssetServer>| {
                setup_splash(loading_time, commands, asset_server)
            },
        )
        .add_system(countdown.run_in_state(self.state_asset_load.clone()))
        .add_exit_system(self.state_asset_load.clone(), despawn_with_recursive::<OnSplashScreen>);
    }
}
