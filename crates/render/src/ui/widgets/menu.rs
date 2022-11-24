use crate::prelude::*;
#[derive(Debug, Component, Default, Eq, PartialEq, Clone)]
pub struct MenuWidget;
impl Widget for MenuWidget {}
#[derive(Bundle)]
pub struct MenuBundle {
    pub styles: KStyle,
    pub children: KChildren,
    pub menu: MenuWidget,
    pub widget_name: WidgetName,
}
impl Default for MenuBundle {
    fn default() -> Self {
        Self {
            menu: Default::default(),
            styles: Default::default(),
            children: Default::default(),
            widget_name: MenuWidget::default().get_name(),
        }
    }
}
