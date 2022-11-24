use crate::prelude::*;
#[derive(Component)]
pub struct UICamera;
pub struct UiPlugin<T: StateNext> {
    pub state_asset_load: T,
    pub state_main_menu: T,
}
impl<T: StateNext + std::default::Default> Plugin for UiPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_plugin(SplashPlugin::new(self.state_asset_load))
            .add_startup_system(spawn_component!((UICameraBundle::default(), UICamera)))
            .add_plugin(MainMenuPlugin {
                state_main_menu: self.state_main_menu,
            });
    }
}
