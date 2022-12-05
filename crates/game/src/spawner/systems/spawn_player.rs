use core::panic;

use crate::prelude::*;

pub fn spawn_player(
    tilesets: Tilesets,
    mut commands: Commands,
    raw_master: Res<RawMaster>,
    mut map_manager: MapManager,
    mut turn_manager: ResMut<TurnManager>,
    q_blocks_movement: Query<&BlocksMovement>,
) {
    let Some(tileset) = tilesets.get_by_id(&TILESET_ACTORS_ID) else {
        // crashing here, may make it hard to chase down other issues?
        error!("Couldn't find tilemap_id: {:?}. Refusing to spawn player.", TILESET_ACTORS_ID);
        return;
    };

    let position = Position::new(
        WorldPosition::ZERO,
        LocalPosition::new(GRID_WIDTH / 2, GRID_HEIGHT / 2, MapLayer::Player as u32),
    );

    raw_master
        .spawn_player_from_raw(
            &mut commands,
            tileset,
            &mut map_manager,
            &q_blocks_movement,
            &position,
        )
        .map_or_else(
            || {
                panic!("Couldn't spawn player");
            },
            |player| {
                turn_manager.add_entity(player);
                commands.insert_resource(PlayerEntity::new(player));
            },
        )
}
