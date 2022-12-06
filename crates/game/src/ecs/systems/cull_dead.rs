use crate::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Dead;

pub fn cull_dead(
    mut commands: Commands,
    mut map_manager: MapManager,
    state: Res<CurrentGameState>,
    player_entity: Res<PlayerEntity>,
    mut turn_manager: ResMut<TurnManager>,
    query: Query<(Entity, &Position, &Name), With<Dead>>,
) {
    for (actor, position, name) in query.iter() {
        map_manager.remove_actor(actor, *position);
        turn_manager.remove_entity(actor);
        commands.entity(actor).despawn_recursive();

        if player_entity.current() == actor {
            println!("YOU DIED!");
            state.go_to(&mut commands, GameState::Dead);
        } else {
            println!("{name} DIED!");
        }
    }
}
