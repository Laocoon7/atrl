use crate::prelude::*;

#[derive(Clone, Component, Debug)]
pub struct CanSeePlayer {
    pub score_if_true: f32,
}

impl Default for CanSeePlayer {
    fn default() -> Self {
        Self { score_if_true: 1.0 }
    }
}

pub fn can_see_player(
    player_q: Query<Entity, With<Player>>,
    mut query: Query<(&Actor, &mut Score)>,
) {
    if let Ok(_player) = player_q.get_single() {
        for (Actor(_actor), mut score) in query.iter_mut() {
            println!("Can see player scorer");

            // Do stuff to see player

            score.set(0.0);
        }
    }
}
