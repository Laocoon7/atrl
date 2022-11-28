use crate::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Dead;

pub fn cull_dead(
    mut commands: Commands,
    mut map_manager: ResMut<MapManager>,
    mut turn_manager: ResMut<TurnManager>,
    query: Query<(&Transform, &Name, Option<&Player>), With<Dead>>,
) {
    for (position, name, player) in query.iter() {
        let Some(map) = map_manager.get_current_map_mut() else {
            error!("No map found :(");
            continue;
        };

        let actor_pos = position.get();

        if let Some(entity) = map.try_remove_actor(actor_pos) {
            turn_manager.remove_entity(entity);
            commands.entity(entity).despawn_recursive();

            if player.is_some() {
                println!("YOU DIED!");
                commands.insert_resource(TurnState::Dead);
            } else {
                println!("{} DIED!", name);
            }
        }
    }
}
