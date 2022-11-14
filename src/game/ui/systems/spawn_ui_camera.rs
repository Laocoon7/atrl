use crate::prelude::*;

pub fn spawn_ui_camera(mut commands: Commands) { commands.spawn(UICameraBundle::new()); }
