use crate::prelude::*;
use big_brain::actions::ActionState;

#[derive(Debug, Default, Clone,)]
pub enum ChaseActorFailureBehavior {
    #[default]
    Wait,
}

#[derive(Debug, Default, Component, Clone,)]
// could be used for temporary storage for multi turn actions
pub struct ChaseActor {
    path: Option<Vec<IVec2,>,>,
    last_seen_pt: Option<IVec2,>,
    // What to do if entity reaches last seen player position
    fail_behavior: ChaseActorFailureBehavior,
}

pub fn chase_action(
    mut commands: Commands,
    mut manager: ResMut<MapManager,>,
    player_q: Query<(Entity, &Transform,), With<Player,>,>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut ChaseActor, &ActionSpan,),>,
    mut ai_q: Query<(&mut Transform, &FieldOfView, &Vision, &Movement, &Name,), Without<Player,>,>,
    mut target_q: Query<&mut TargetVisualizer,>,
    tilesets: Tilesets,
) {
    use ActionState::*;

    for (Actor(actor,), mut action_state, mut chase, span,) in action_q.iter_mut() {
        let _guard = span.span().enter();

        let (_player_entity, player_transform,) = player_q.single();
        let (mut position, fov, vision, movement_component, name,) =
            ai_q.get_mut(*actor,).expect("Actor must have required components",); // TODO: Do we really want to crash the game here or just `error!` `return`?

        let ai_pos = position.get();
        let player_pos = player_transform.get();

        let map = manager.get_current_map_mut().expect("No map found",); // TODO: Do we really want to crash the game here or just `error!` `return`?

        // This doesnt exist on the original match below because whenever the player comes into view
        // the `can_see_player` scorer sets the output to be 1, causing the wander action to spin
        // down (1 frame). Then, the player moves and its the ai turn again. the chase
        // actions boots up and evaluates (2 frames). After that spin up, when the player,
        // finally moves a 3rd time, the chase action moves into the Executing
        // state and moves the AI towards the player.
        //
        // So this acts like a skip frame, where it sets the action to evaluating, then immediately
        // evaluates
        if *action_state == Requested {
            chase.last_seen_pt = Some(player_pos,);
            chase.path = Some(generate_chase_path(ai_pos, player_pos, movement_component.0, map,),);

            *action_state = Executing;
            info!("{} gonna start chasing!", name);
        }

        match *action_state {
            Cancelled => {
                if let Ok(mut target_visualizer,) = target_q.get_mut(*actor,) {
                    target_visualizer.clear(&mut commands,);
                }
                info!("{} cancelled chase!", name);
                *action_state = Failure;
            }
            Executing => {
                info!("{} executing chase!", name);

                /*
                                let update_path_target = if entity_in_fov(map, fov, vision, ai_pos, player_pos) {
                                    // we saw the player, update the last seen position
                                    chase.last_seen_pt = Some(player_pos);
                                    if false { // can_attack(player_position) {
                                        // We should attack instead of moving!
                                        *actions_state = ActionState::Failure;
                                        return;
                                    }
                                    // always update when we can see the player
                                    // so treat it as we don't have a valid path
                                    Some(player_pos)
                                } else {
                                    let Some(last_seen_position) = chase.last_seen_pt else {
                                        // How did we get here?
                                        error!("AI is chasing, but it has no last_seen_position.");
                                        *action_state = ActionState::Failure;
                                        return;
                                    };

                                    // Do we have a place we are chasing to?
                                    if let Some(path) = chase.path {
                                        if path.len() == 0 {
                                            // we don't have a valid path because:
                                            // we are at the end of the chase, and we don't see the player.
                                            //
                                            // SWITCH TO WANDER_STATE
                                            //
                                            // We can technically still move towards this point here
                                            // but I'm not sure how to switch from chase -> wander
                                            // I'll try this for now:
                                            *action_state = ActionState::Failure;
                                            return;
                                        } else if map.can_place_actor(path[path.len() - 1], movement_component.0) {
                                            // we have a valid path, and the next step is also valid!
                                            // we only check to make sure this is valid to see if we need to
                                            // try re-generating a path. this move will be checked again
                                            // as we actually try to move there.
                                            None
                                        } else {
                                            // we have a valid path
                                            // but something is blocking us.. Actor/New Feature/Etc
                                            // update the path to try to get around this thing...
                                            Some(last_seen_position)
                                        }
                                    }
                                };

                                if let Some(target_position) = update_path_target {
                                    chase.path = Some(generate_chase_path(ai_pos, target_position, movement_component.0, map));
                                }

                                let Some(chase_path) = std::mem::take(&mut chase.path) else {
                                    // previous update path failed...
                                    error!("AI could not find a path for chasing.");
                                    *action_state = ActionState::Failure;
                                    return;
                                };
                */
                //************************
                //************************
                //************************
                let target_pt = if entity_in_fov(map, fov, vision, ai_pos, player_pos) {
                    // update last_seen_pt incase we lose sight of the player next time
                    chase.last_seen_pt = Some(player_pos); // this could be more consistent from every state if it has it's own system that runs once per turn
                                                           // if path.len() <= desired_attack_range {
                                                           // we are in range to attack
                                                           // set state to AttackState
                                                           // return from function; // we don't handle attacking here
                                                           // }
                                                           // updating our path should be next separate from getting the target_pt
                                                           //should_update_path = true; // we want to update the path if we see the player because they can move.
                    chase.path =
                        Some(generate_chase_path(ai_pos, player_pos, movement_component.0, map));
                    // return the player_pos to generate a path to
                    Some(player_pos)
                } else {
                    // we can't see the player, find somewhere to go:
                    // if path.len() == 0 {
                    // we hit the end of chasing.
                    // set state to WanderState
                    // return a point to wander to but this frame, movement can still be handled here
                    // }
                    // if path.len() > 0 && !map.can_place_actor(path[path.len() - 1], movement_component.0) {
                    //should_update_path = false; // we have a place to go and are able to do so already.
                    // else {
                    //should_update_path = true; // we will want to
                    // }

                    chase.last_seen_pt
                };

                // We should update the path to target_pt here

                // With the above changes, we have a path that is len() > desired_attack_distance
                // and we already took care of finding a place to move to. This wouldn't be necessary.
                let Some(target_pt) = target_pt else {
                    // Currently:
                    // This runs if chase.last_seen_pt was `None`
                    // can action_state be set to Executing without setting a last_seen_pt?
                    // Should this be an `error!` instead of `info`?
                    // or can it happen and is fine?
                    info!("Target not seen, what was I chasing?");
                    *action_state = ActionState::Failure;
                    return;
                };

                // With the above changes, we have a path with len() > desired_attack_distance
                // target_pt will never be ai_pos at this point. This wouldn't be necessary.
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

                // This should happen when updating the path at the top. When updating the path
                let mut chase_path = match std::mem::take(&mut chase.path) {
                    Some(p) => {
                        //p, // use this at the top as the rest of this is uneeded then
                        if p.is_empty() || !map.can_place_actor(p[0], movement_component.0) {
                            generate_chase_path(ai_pos, target_pt, movement_component.0, map)
                        } else {
                            p
                        }
                    }
                    None => generate_chase_path(ai_pos, target_pt, movement_component.0, map),
                };
                //let mut chase_path = chase_path
                //    .and_then(|p| {
                //        if p.is_empty() || !map.can_place_actor(p[0], movement_component.0) {
                //            None
                //        } else {
                //            Some(p)
                //        }
                //    })
                //    .unwrap_or_else(|| {
                //        generate_chase_path(&ai_pos, &target_pt, movement_component.0, map)
                //    });
                //************************
                //************************
                //************************

                // We have a path > 1 and we are not in range to attack.
                println!("Chase path: {:?}", chase_path);
                // if let Some(next_pt) = chase_path.pop() { // we might be blocked and can't move do to another actor in our way
                let next_pt = chase_path[chase_path.len() - 1];
                // we are only worried about how many path points there are
                //
                // If path.len() == 0 && we don't see the player, the path would
                // have been regenerated and we would already be switched to
                // WanderState.
                //
                // If path.len() == 0 && we see the player, the path would
                // have been regenerated and we would have already gone through
                // a path.len() == 1 iteration.
                //
                // If path.len() == 1 && we see the player, he is next to
                // us and we should already be switched to attacking.
                //
                // If path.len() == 1 && we don't see the player, we need to
                // finish moving to last known point.
                //
                // Realistically, this gets here *ONLY* when
                // path.len() > 1 and we still can't attack
                // if we see player: path[0] is player_pos, and path[1] is somewhere to move to
                // if we don't see the player: path[0] is a valid move, and path[1] is a valid move.
                // valid only means we *WANT* to move there, not that we are physically able
                //
                // Even though we are guaranteed to have at least a single valid position,
                // if we were to check again here:
                //
                // if chase_path.len() > 1 is WAAAYYYY more effecient than:
                //
                // convert target_pt to Vec2
                // convert ai_pos to Vec2
                // let max_x = if target_pt.x > ai_pos.x {target_pt.x} else {ai_pos.x};
                // let max_y = if target_pt.y > ai_pos.y {target_pt.y} else {ai_pos.y};
                // let min_x = if target_pt.x < ai_pos.x {target_pt.x} else {ai_pos.x};
                // let min_y = if target_pt.y < ai_pos.y {target_pt.y} else {ai_pos.y};
                // let sub = Vec2::new((max_x - min_x), (max_y - min_y));
                // let square = sub.powf(2); // worse performance than powi() on an integer
                // let root = square.sqrt(); // You have a full DistanceAlg to avoid this one call if possible because it's so bad.
                // if root > DIAGONAL_COST
                //
                // DIAGONAL_COST is only there to allow A* to favor
                // not checking 3 positions (NW / N / NE) when they will
                // all give the same result. EG:
                //
                // ai_pos = (10, 10)
                // target_pt = (10, 12)
                //  .X.
                //  ...
                //  .O.
                // valid paths:
                // (9, 11) -> (10, 12)
                // (10, 11) -> (10, 12)
                // (11, 11) -> (10, 12)
                //
                //  .X.
                //  |||
                //  .O.
                // they are all the same distance move wise
                // but we only want the straight on path
                // so we assign a DIAGONAL_COST (CARDINAL_COST + f32::epsillon -> CARDINAL_COST * 2 - f32::epsillon)
                // in order to not get these other results.
                let distance = DistanceAlg::Pythagoras.distance2d(ai_pos, target_pt);
                println!("Distance to next point: {} {}", distance, DIAGONAL_COST);
                //if distance > DIAGONAL_COST {
                if map.try_move_actor(ai_pos, next_pt, movement_component.0) {
                    position.set_value(next_pt);
                    if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                        if chase_path.len() > 0 {
                            target_visualizer.update(
                                &mut commands,
                                &tilesets,
                                next_pt,
                                chase_path[0],
                                Color::RED,
                            );
                        } else {
                            target_visualizer.update(
                                &mut commands,
                                &tilesets,
                                next_pt,
                                next_pt,
                                Color::RED,
                            );
                        }
                    }
                    chase_path.pop(); // consume the path point
                } else {
                    info!("AI is blocked trying to move from {:?} to {:?}", ai_pos, next_pt);
                }
                //} else {
                // We cannot get closer to the player, so we just wait
                // *action_state = Success;
                //}

                chase.path = Some(chase_path);
            }

            // Init | Success | Failure
            _ => {}
        }

        info!("Chase action output: {:?}\n", action_state);
    }
}

fn generate_chase_path(
    ai_pos: IVec2, // removed & they are copy
    target_pos: IVec2,
    movement_type: u8,
    map_provider: &impl PathProvider,
) -> Vec<IVec2,> {
    PathFinder::Astar.compute(ai_pos, target_pos, movement_type, map_provider,).unwrap_or_default()
}
