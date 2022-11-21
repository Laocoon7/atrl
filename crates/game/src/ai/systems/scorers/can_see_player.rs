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
    manager: Res<MapManager>,
    player_q: Query<(Entity, &Transform), With<Player>>,
    ai_q: Query<(&Transform, &FieldOfView, &Vision)>,
    mut query: Query<(&Actor, &mut Score, &CanSeePlayer)>,
) {
    for (_player, player_transform) in &player_q {
        for (Actor(actor), mut score, can_see_player) in query.iter_mut() {
            let mut sees_player = false;
            if let Ok((ai_transform, fov, vision)) = ai_q.get(*actor) {
                if let Some(map) = manager.get_current_map() {
                    let player_pos = player_transform.get();
                    let ai_pos = ai_transform.get();

                    // If the player is within the FOV range of the AI, check line of sight
                    if DistanceAlg::Pythagoras.distance2d(ai_pos, player_pos) < fov.0 as f32 {
                        let line = Line::new(ai_pos, player_pos);
                        if line.iter().all(|point| map.can_see_through(point, vision)) {
                            score.set(can_see_player.score_if_true);
                            sees_player = true;
                        }
                    }
                }
            }

            if !sees_player {
                score.set(0.0);
            }
        }
    }
}
