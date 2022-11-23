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
    path: Option<Vec<IVec2>>,
    last_seen_pt: Option<IVec2>,
    // What to do if entity reaches last seen player position
    fail_behavior: ChaseActorFailureBehavior,
}

pub fn chase_action(
    manager: Res<MapManager>,
    player_q: Query<(Entity, &Transform), With<Player>>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut ChaseActor, &ActionSpan)>,
    mut ai_q: Query<(&mut Transform, &FieldOfView, &Vision, &Movement, &Name), Without<Player>>,
) {
    use ActionState::*;

    for (Actor(actor), mut action_state, mut chase, span) in action_q.iter_mut() {
        let _guard = span.span().enter();

        let (_player_entity, player_transform) = player_q.single();
        let (mut position, fov, vision, movement_component, name) =
            ai_q.get_mut(*actor).expect("Actor must have required components");

        let ai_pos = position.get();
        let player_pos = player_transform.get();

        let map = manager.get_current_map().expect("No map found");

        // This doesnt exist on the original match below because whenever the player comes into view
        // the `can_see_player` scorer sets the output to be 1, causing the wander action to spin down (1 frame).
        // Then, the player moves and its the ai turn again. the chase actions boots up and evaluates (2 frames).
        // After that spin up, when the player, finally moves a 3rd time, the chase action moves into the Executing
        // state and moves the AI towards the player.
        //
        // So this acts like a skip frame, where it sets the action to evaluating, then immediately evaluates
        if *action_state == Requested {
            chase.last_seen_pt = Some(player_pos);
            chase.path = Some(generate_chase_path(&ai_pos, &player_pos, movement_component.0, map));

            *action_state = Executing;
            info!("{} gonna start chasing!", name);
        }

        match *action_state {
            Cancelled => {
                info!("{} cancelled chase!", name);
                *action_state = Failure;
            }
            Executing => {
                info!("{} executing chase!", name);
                let target_pt = if entity_in_fov(map, fov, vision, ai_pos, player_pos) {
                    chase.last_seen_pt = Some(player_pos);
                    chase.path =
                        Some(generate_chase_path(&ai_pos, &player_pos, movement_component.0, map));
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

                    let chase_path = std::mem::take(&mut chase.path);
                    let mut chase_path = chase_path
                        .and_then(|p| {
                            if p.is_empty() || !map.can_move_through(p[0], movement_component.0) {
                                None
                            } else {
                                Some(p)
                            }
                        })
                        .unwrap_or_else(|| {
                            generate_chase_path(&ai_pos, &target_pt, movement_component.0, map)
                        });

                    println!("Chase path: {:?}", chase_path);
                    if let Some(next_pt) = chase_path.pop() {
                        let distance = DistanceAlg::Pythagoras.distance2d(ai_pos, target_pt);
                        println!("Distance to next point: {} {}", distance, DIAGONAL_COST);
                        if distance > DIAGONAL_COST {
                            position.set_value(next_pt);
                        } else {
                            // We cannot get closer to the player, so we just wait
                            // *action_state = Success;
                        }
                    }

                    chase.path = Some(chase_path);
                } else {
                    info!("Target not seen, what was I chasing?");
                    *action_state = ActionState::Failure;
                }
            }

            // Init | Success | Failure
            _ => {}
        }

        info!("Chase action output: {:?}\n", action_state);
    }
}

fn generate_chase_path(
    ai_pos: &IVec2,
    target_pos: &IVec2,
    movement_type: u8,
    map_provider: &impl PathProvider,
) -> Vec<IVec2> {
    PathFinder::Astar.compute(*ai_pos, *target_pos, movement_type, map_provider).unwrap_or_default()
}
