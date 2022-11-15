use crate::prelude::*;

#[derive(Debug, Component, Default, Eq, PartialEq, Clone)]
pub struct MainMenuState {
    pub show: bool,
}

pub struct MainMenuPlugin<T> {
    pub state_main_menu: T,
}

impl<T: StateNext> Plugin for MainMenuPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.state_main_menu.clone(), setup_main_menu)
            .add_exit_system(self.state_main_menu.clone(), remove_from_all::<KayakRootContext>)
            .add_exit_system(self.state_main_menu.clone(), despawn_with_recursive::<KStyle>);
    }
}
