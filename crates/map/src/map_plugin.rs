use crate::prelude::*;

pub struct MapPlugin<T> {
    pub state_construct: T,
    pub state_running: T,
}

impl<T: StateData> Plugin for MapPlugin<T> {
    fn build(&self, _app: &mut App) {}
}
