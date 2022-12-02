use std::f32::consts::PI;

use atrl_common::prelude::grid_shapes::GridShape;
use num_traits::real::Real;

use crate::prelude::*;
#[derive(Clone, Component, Debug)]
pub struct CanSeePlayer {
    pub score_if_true: f32,
}

impl Default for CanSeePlayer {
    fn default() -> Self { Self { score_if_true: 1.0 } }
}

pub fn can_see_player(
    manager: Res<MapManager>,
    ai_q: Query<(&Position, &FieldOfView, &Vision)>,
    player_q: Query<(Entity, &Position), With<Player>>,
    mut query: Query<(&Actor, &mut Score, &CanSeePlayer)>,
) {
    for (_player, player_position) in &player_q {
        for (Actor(actor), mut score, can_see_player) in query.iter_mut() {
            let mut current_score = 0.0;

            if let Ok((ai_transform, fov, vision)) = ai_q.get(*actor) {
                if let Some(map) = manager.get_current_map() {
                    let player_pos = player_position.gridpoint();
                    let ai_pos = ai_transform.gridpoint();
                    if entity_in_fov(map, fov, vision, ai_pos, player_pos) {
                        current_score = can_see_player.score_if_true;
                    }
                }
            }

            score.set(current_score);
        }
    }
}
