use atrl_common::prelude::grid_shapes::GridShape;

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
    player_q: Query<(Entity, &Transform), With<Player>>,
    ai_q: Query<(&Transform, &FieldOfView, &Vision)>,
    mut query: Query<(&Actor, &mut Score, &CanSeePlayer)>,
) {
    for (_player, player_transform) in &player_q {
        for (Actor(actor), mut score, can_see_player) in query.iter_mut() {
            let mut current_score = 0.0;
            if let Ok((ai_transform, fov, vision)) = ai_q.get(*actor) {
                if let Some(map) = manager.get_current_map() {
                    let player_pos = player_transform.get();
                    let ai_pos = ai_transform.get();

                    // If the player is within the FOV range of the AI, check line of sight
                    let line = grid_shapes::Line::new(ai_pos, player_pos).get_points();
                    if line.len() < fov.0 as usize {
                        current_score = can_see_player.score_if_true;
                        for point in line {
                            if map.is_opaque(point, vision.0) {
                                current_score = 0.0;
                                break;
                            }
                        }
                    }
                }
            }
            score.set(current_score);
        }
    }
}
