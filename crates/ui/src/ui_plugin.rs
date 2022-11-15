use crate::prelude::*;

pub struct UiPlugin<T> {
    pub state_main_menu: T,
}

impl<T: StateNext> Plugin for UiPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_startup_system(spawn_ui_camera)
            .add_startup_system(setup_kayak_ui)
            .add_plugin(MainMenuPlugin { state_main_menu: self.state_main_menu.clone() });
    }
}
