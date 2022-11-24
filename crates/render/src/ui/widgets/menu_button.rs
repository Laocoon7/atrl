use crate::prelude::*;
#[derive(Default, Clone, Eq, PartialEq, Component)]
pub struct MenuButton {
    pub text: String,
}
impl Widget for MenuButton {}
#[derive(Bundle)]
pub struct MenuButtonBundle {
    pub styles: KStyle,
    pub button: MenuButton,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for MenuButtonBundle {
    fn default() -> Self {
        Self {
            button: Default::default(),
            styles: KStyle {
                bottom: Units::Pixels(20.0).into(),
                cursor: KCursorIcon(CursorIcon::Hand).into(),
                ..Default::default()
            },
            on_event: OnEvent::default(),
            widget_name: MenuButton::default().get_name(),
        }
    }
}
