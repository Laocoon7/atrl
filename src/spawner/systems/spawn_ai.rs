use crate::prelude::*;

const MAX_AI_ACTORS: u32 = 1;

pub fn spawn_ai(
    tilesets: Tilesets,
    mut commands: Commands,
    raw_master: Res<RawMaster>,
    mut map_manager: MapManager,
    state: Res<CurrentGameState>,
    mut turn_manager: ResMut<TurnManager>,
    q_blocks_movement: Query<&BlocksMovement>,
) {
    let Some(tileset) = tilesets.get_by_id(&TILESET_ACTORS_ID) else {
        // let's not crash the program just because we can't spawn an ai.
        error!("Couldn't find tilemap_id: {:?}. Refusing to spawn ai.", TILESET_ACTORS_ID);
        return;
    };

    let mut actor_count = 0;
    for y in 0..MAX_AI_ACTORS {
        for x in 0..MAX_AI_ACTORS {
            if actor_count >= MAX_AI_ACTORS {
                break;
            }

            let position = Position::new(
                WorldPosition::ZERO,
                LocalPosition::new(
                    GRID_WIDTH / 3 + x,
                    GRID_HEIGHT / 3 + y,
                    MapLayer::Actors as u32,
                ),
            );

            if let Some(ai_entity) = raw_master.spawn_mob_from_raw(
                &mut commands,
                tileset.atlas(),
                &mut map_manager,
                &q_blocks_movement,
                position,
                "Gary",
            ) {
                turn_manager.add_entity(ai_entity);
                actor_count += 1;
            }
        }
    }

    state.set_next(&mut commands);
}
