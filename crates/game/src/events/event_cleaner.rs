use crate::prelude::*;
pub(crate) fn event_cleaner<T: 'static + Send + Sync>(mut events: ResMut<Events<T>>) {
    // events.clear();
    // for event in events.drain() {
    //
    //}
    events.update();
}
