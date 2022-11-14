use crate::prelude::*;

pub fn setup_main_menu(mut query: Query<&mut MainMenuState, Without<PreviousWidget>>) {
    for mut my_widget in query.iter_mut() {
        my_widget.show = true;
    }
}
