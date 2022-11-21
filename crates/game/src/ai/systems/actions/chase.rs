use crate::prelude::*;
use big_brain::actions::ActionState;

#[derive(Debug, Default, Clone)]
pub enum ChaseActorFailureBehavior {
    #[default]
    Wait,
}

#[derive(Debug, Default, Component, Clone)]
// could be used for temporary storage for multi turn actions
pub struct ChaseActor {
    last_seen_pt: Option<Vec2>,
    // What to do if entity reaches last seen player position
    fail_behavior: ChaseActorFailureBehavior,
}

pub fn chase_action(
    manager: Res<MapManager>,
    player_q: Query<(Entity, &Transform), With<Player>>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut ChaseActor)>,
    mut ai_q: Query<(&mut Transform, &FieldOfView, &Vision, &Movement), Without<Player>>,
) {
    use ActionState::*;

    for (Actor(actor), mut action_state, mut chase) in action_q.iter_mut() {
        let (_player_entity, player_transform) = player_q.single();

        match *action_state {
            ActionState::Init => {
                println!("Chase init");
                continue;
            }
            ActionState::Success => {
                println!("Chase success");
                continue;
            }
            ActionState::Cancelled => {
                println!("Chase cancelled");
                *action_state = ActionState::Failure;
                continue;
            }
            ActionState::Failure => {
                println!("Chase failed");
                continue;
            }
            // these final two fall through to logic
            ActionState::Requested => {
                println!("I'm gonna start chasing");
                chase.last_seen_pt = Some(player_transform.get());
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {}
        }

        if let Ok((mut position, fov, vision, movement_component)) = ai_q.get_mut(*actor) {
            if let Some(map) = manager.get_current_map() {
                let ai_pos = position.get();
                let player_pos = player_transform.get();

                let sees_player = entity_in_fov(map, fov, vision, ai_pos, player_pos);
                let target_pt = if sees_player {
                    chase.last_seen_pt = Some(player_pos);
                    Some(player_pos)
                } else {
                    chase.last_seen_pt
                };

                if let Some(target_pt) = target_pt {
                    if target_pt == ai_pos {
                        // We reached the end of our trail and the target isn't nearby
                        // Let's do fallback behavior until AI cancels us
                        match chase.fail_behavior {
                            ChaseActorFailureBehavior::Wait => {
                                println!("Reached last known position of player. Just gonna wait.");
                            }
                        }
                        *action_state = ActionState::Failure;
                        continue;
                    }

                    if let Some(path) =
                        PathFinder::Astar.compute(ai_pos, target_pt, movement_component, map)
                    {
                        let distance = DistanceAlg::Pythagoras.distance2d(ai_pos, target_pt);
                        info!("Path: {:?} / distance: {}", path, distance);
                        if distance > 1.45 {
                            let destination = path.0[1].as_vec2() + Vec2::new(0.5, 0.5);
                            info!("Moving from {:?} to {:?}", ai_pos, destination);
                            position.set_value(destination);
                        } else {
                            // We can't get closer
                            info!("Can't get closer to player");
                            *action_state = ActionState::Success;
                        };
                    }
                } else {
                    info!("Target not seen, what was I chasing?");
                    *action_state = ActionState::Failure;
                }
            } else {
                *action_state = ActionState::Failure;
            }
        } else {
            *action_state = ActionState::Failure;
        }

        info!("Chase action output: {:?}\n", action_state);
    }
}
