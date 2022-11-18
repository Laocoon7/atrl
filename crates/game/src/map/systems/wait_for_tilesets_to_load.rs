use crate::prelude::*;

pub(crate) fn wait_for_tilesets_to_load(
    mut commands: Commands,
    tilesets: Tilesets,
    state: Res<CurrentGameState>,
) {
    let tilesets_count = tilesets.len() as u8;

    if tilesets_count == 0 {
        return;
    }

    for i in 0..tilesets_count {
        if tilesets.get_by_id(&i).is_none() {
            info!("Waiting for {} tilesets to load...", tilesets_count);
            return;
        }
    }

    state.set_next(&mut commands);
}
