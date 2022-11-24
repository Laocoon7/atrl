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
        let Ok((mut position, fov, vision, movement_component, name,)) =
            ai_q.get_mut(*actor,) else {
                error!("Actor must have required components");
                return
            };

        let ai_pos = position.get();
        let player_pos = player_transform.get();

        let Some(map) = manager.get_current_map_mut() else {
            error!("No map found");
            return
        };

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

                // if update_path_target is_some() update the path
                // otherwise we will assume chase.path is valid
                let update_path_target = if entity_in_fov(map, fov, vision, ai_pos, player_pos,) {
                    // we saw the player, update the last seen position
                    chase.last_seen_pt = Some(player_pos,);
                    if can_attack(player_pos,) {
                        // We should attack instead of moving!
                        *action_state = ActionState::Failure;
                        return;
                    }
                    // always update when we can see the player
                    // so treat it as we don't have a valid path
                    Some(player_pos,)
                } else {
                    let Some(last_seen_position) = chase.last_seen_pt else {
                        // How did we get here?
                        // Make sure every transfer into chase is accompanied
                        // by chase.last_seen_pt being set!
                        error!("AI is chasing, but it has no last_seen_position.");
                        *action_state = ActionState::Failure;
                        return;
                    };

                    Some(last_seen_position,)
                };

                // update the path if necessary!
                if let Some(target_position,) = update_path_target {
                    chase.path = Some(generate_chase_path(
                        ai_pos,
                        target_position,
                        movement_component.0,
                        map,
                    ),);
                }

                let Some(mut chase_path) = std::mem::take(&mut chase.path) else {
                    // previous update path failed...
                    error!("AI could not find a path for chasing.");
                    *action_state = ActionState::Failure;
                    return;
                };

                // We have a path > 1 and we are not in range to attack.
                println!("Chase path: {:?}", chase_path);
                let next_pt = chase_path[chase_path.len() - 1]; // wish we had a Vec::peek()
                if map.try_move_actor(ai_pos, next_pt, movement_component.0,) {
                    // we were moved!
                    position.set_value(next_pt,);
                    if let Ok(mut target_visualizer,) = target_q.get_mut(*actor,) {
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

                chase.path = Some(chase_path,);
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

fn can_attack(position: IVec2,) -> bool { false }
