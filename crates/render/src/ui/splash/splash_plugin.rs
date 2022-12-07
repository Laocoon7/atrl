use crate::prelude::*;
#[derive(SmartDefault)]
pub struct SplashPlugin<T: StateNext> {
    #[default(1.0)]
    pub loading_time: f32,
    pub state_asset_load: T,
}

impl<T: StateNext> SplashPlugin<T> {
    pub fn new(state_asset_load: T) -> Self {
        Self {
            state_asset_load,
            ..Default::default()
        }
    }
}

impl<T: StateNext> Plugin for SplashPlugin<T> {
    fn build(&self, app: &mut App) {
        let loading_time = self.loading_time;

        app.add_enter_system(
            self.state_asset_load,
            move |commands: Commands, asset_server: Res<AssetServer>, font_mapping: ResMut<FontMapping>| {
                setup_splash(loading_time, commands, asset_server, font_mapping)
            },
        )
        .add_system(countdown.run_in_state(self.state_asset_load))
        .add_exit_system(
            self.state_asset_load,
            despawn_with_recursive::<OnSplashScreen>,
        );
    }
}
