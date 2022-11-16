use crate::prelude::*;

#[derive(Debug)]
pub struct WantsToMove(pub Entity, pub Vec2);

pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) { app.add_event::<WantsToMove>(); }
}
