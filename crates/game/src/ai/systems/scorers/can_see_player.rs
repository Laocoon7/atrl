use crate::prelude::*;

#[derive(Clone, Component, Debug)]
pub struct CanSeePlayer {
    pub score_if_true: f32,
}

impl Default for CanSeePlayer {
    fn default() -> Self { Self { score_if_true: 1.0 } }
}

pub fn can_see_player<'w, 's>(
    mut map_manager: MapManager,
    mobs_q: Query<(&Position, &FieldOfView, &Vision)>,
    player_entity: Res<PlayerEntity>,
    mut query: Query<(&Actor, &mut Score, &CanSeePlayer)>,
    q_blocks_vision: Query<'w, 's, &'static BlocksVision>,
) {
    let Ok((player_position, ..)) = mobs_q.get(player_entity.current()) else {
        error!("No player!");
        return;
    };

    for (Actor(actor), mut score, can_see_player) in query.iter_mut() {
        if *actor == player_entity.current() {
            continue;
        }
        let mut current_score = 0.0;

        if let Ok((ai_position, fov, vision)) = mobs_q.get(*actor) {
            if entity_in_fov(
                &mut map_manager,
                &q_blocks_vision,
                fov.0 as u32,
                vision,
                *ai_position,
                *player_position,
            ) {
                current_score = can_see_player.score_if_true;
            }
        }

        score.set(current_score);
    }
}
