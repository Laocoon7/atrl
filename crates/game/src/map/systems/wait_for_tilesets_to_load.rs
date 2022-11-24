use crate::prelude::*;
pub fn wait_for_tilesets_to_load(
    mut commands: Commands,
    tilesets: Tilesets,
    state: Res<CurrentGameState>,
) {
    for i in TILESET_ACTORS_IDS_TABLE {
        if tilesets.get_by_id(i).is_none() {
            info!("Waiting for actor tilesets to load...");
            return;
        }
    }
    for i in TILESET_FEATURES_IDS_TABLE {
        if tilesets.get_by_id(i).is_none() {
            info!("Waiting for feature tilesets to load...");
            return;
        }
    }
    for i in TILESET_ITEMS_IDS_TABLE {
        if tilesets.get_by_id(i).is_none() {
            info!("Waiting for item tilesets to load...");
            return;
        }
    }
    for i in TILESET_TERRAIN_IDS_TABLE {
        if tilesets.get_by_id(i).is_none() {
            info!("Waiting for terrain tilesets to load...");
            return;
        }
    }
    state.set_next(&mut commands);
}
