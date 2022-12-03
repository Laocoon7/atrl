use crate::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Dead;

pub fn cull_dead(
    mut commands: Commands,
    state: Res<CurrentGameState>,
    mut map_manager: ResMut<MapManager>,
    mut turn_manager: ResMut<TurnManager>,
    query: Query<(Entity, &Position, &Name, Option<&Player>), With<Dead>>,
) {
    for (entity, position, name, player) in query.iter() {
        let Some(map) = map_manager.get_current_map_mut() else {
            error!("No map found :(");
            continue;
        };

        let actor_pos = position.gridpoint();
        map.try_remove_actor(actor_pos).map_or_else(
            || {
                error!(
                    "[Entity: {:?}] {} does not exist at {:?}",
                    entity, name, actor_pos
                );
            },
            |entity| {
                turn_manager.remove_entity(entity);
                commands.entity(entity).despawn_recursive();

                if player.is_some() {
                    println!("YOU DIED!");
                    state.go_to(&mut commands, GameState::Dead);
                } else {
                    println!("{} DIED!", name);
                }
            },
        )
    }
}
