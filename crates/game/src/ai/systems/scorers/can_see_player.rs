use crate::prelude::*;

pub fn can_see_player(
    player_q: Query<Entity, With<Player>>,
    mut query: Query<(&Actor, &mut Score)>,
) {
    if let Ok(_player) = player_q.get_single() {
        for (Actor(_actor), mut _score) in query.iter_mut() {
            // Do stuff to see player
        }
    }
}
