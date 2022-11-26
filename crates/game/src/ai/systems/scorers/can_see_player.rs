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
    ai_q: Query<(&Transform, &FieldOfView, &Vision)>,
    player_q: Query<(Entity, &Transform), With<Player>>,
    mut query: Query<(&Actor, &mut Score, &CanSeePlayer)>,
) {
    for (_player, player_transform) in &player_q {
        for (Actor(actor), mut score, can_see_player) in query.iter_mut() {
            let mut current_score = 0.0;

            if let Ok((ai_transform, fov, vision)) = ai_q.get(*actor) {
                if let Some(map) = manager.get_current_map() {
                    let player_pos = player_transform.get();
                    let ai_pos = ai_transform.get();

                    // FIX: This was the normal way. Checking if the player is in the fov,
                    // then drawing a line from ai -> player and checking if any tiles are
                    // is_opaque
                    // |
                    // |
                    // V

                    // If the player is within the FOV range of the AI, check line of sight
                    // if entity_in_fov(map, fov, vision, ai_pos, player_pos) {
                    //     current_score = can_see_player.score_if_true;
                    // }

                    // Second way, check if the player is within the FOV range of the AI,
                    // then use fov direction to check all points along the line for visibility
                    let line_length = grid_shapes::Line::new(ai_pos, player_pos).get_count();
                    if line_length < fov.0 as usize {
                        let mut visibility_map = VisibilityMap::new(map.size);
                        let angle = (player_pos.angle_to(ai_pos) - 180.0).abs();
                        Fov::ShadowcastDirection(CardinalDirection::from(angle as i32)).compute(
                            ai_pos,
                            vision.0,
                            fov.0,
                            map,
                            &mut visibility_map,
                        );

                        // FIX: Err I hate having to skip the first tile
                        if Line::new(ai_pos, player_pos)
                            .iter()
                            .skip(1)
                            .all(|pt| visibility_map.get_visible(pt))
                        {
                            current_score = can_see_player.score_if_true;
                        }
                    }
                }
            }

            score.set(current_score);
        }
    }
}
