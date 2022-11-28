use std::{collections::VecDeque, ops::RangeInclusive};

use bevy::render::once_cell::sync::Lazy;
use big_brain::actions::ActionState;
use rand::distributions::Uniform;

use crate::prelude::*;

static WANDER_RANGE: Lazy<Uniform<u32>> = Lazy::new(|| Uniform::new_inclusive(3, 10));

#[derive(Debug, Reflect, Default, Clone, PartialEq, Eq)]
pub enum WanderFailureBehavior {
    #[default]
    Wait,
}

// could be used for temporary storage for multi turn actions
#[derive(Debug, Reflect, Default, Component, Clone, Eq, PartialEq)]
#[reflect(Component)]
pub struct Wander {
    path: Option<Vec<IVec2>>,
    // What to do if entity has no available places to move
    fail_behavior: WanderFailureBehavior,
}

pub fn wander_action(
    tilesets: Tilesets,
    mut commands: Commands,
    mut ctx: ResMut<GameContext>,
    mut manager: ResMut<MapManager>,
    mut target_q: Query<&mut TargetVisualizer>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut Wander, &ActionSpan)>,
    mut spatial_q: Query<(&Transform, &Movement, &Name, &mut AIComponent), Without<Player>>,
) {
    use ActionState::*;

    for (Actor(actor), mut action_state, mut wander, span) in action_q.iter_mut() {
        let _guard = span.span().enter();

        let rng = ctx.random.get_prng().as_rngcore();
        let Some(map) = manager.get_current_map_mut() else {
            error!("No map found");
            return
        };

        let Ok((position, movement_component, name, mut ai_component)) =
        spatial_q.get_mut(*actor) else {
                error!("Actor must have spatial components");
                return
            };

        if ai_component.preferred_action.is_some() {
            // already wandering, quick return;
            commands.insert_resource(TurnState::Processing);
            return;
        }

        let ai_pos = position.get();

        match *action_state {
            Cancelled => {
                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }
                *action_state = Failure;
            },
            Requested => {
                info!("{} gonna start wandering!", name);
                *action_state = Executing;
                wander.path = generate_wander_path(rng, ai_pos, movement_component.0, map);
            },
            Executing => {
                let wander_path = std::mem::take(&mut wander.path);
                let ai_path = wander_path.and_then(|p| {
                    if p.is_empty() || !map.can_place_actor(p[0], movement_component.0) {
                        if let Some(path) = generate_wander_path(rng, ai_pos, movement_component.0, map) {
                            if path.len() == 0 {
                                None
                            } else {
                                Some(path)
                            }
                        } else {
                            None
                        }
                    } else {
                        Some(p)
                    }
                });

                if let Some(mut ai_path) = ai_path {
                    if map.can_place_actor(ai_path[0], movement_component.0) {
                        ai_path.pop().map_or_else(
                            || {
                                // We have reached the end of our trail!
                                *action_state = Success;
                                info!("{} has reached the end of their wander path!", name);
                            },
                            |next_pt| {
                                update_target_visual(
                                    &mut commands,
                                    &tilesets,
                                    &mut target_q,
                                    &ai_path,
                                    actor,
                                    &next_pt,
                                    Color::YELLOW,
                                );
                                ai_component.preferred_action = Some(ActionType::Movement(next_pt));
                                // move_events.send(WantsToMove(*actor, next_pt));
                            },
                        );

                        wander.path = Some(ai_path);
                    } else {
                        ai_component.preferred_action = Some(ActionType::Wait);
                    }
                } else {
                    ai_component.preferred_action = Some(ActionType::Wait);
                }
                commands.insert_resource(TurnState::Processing);
            },

            // Init | Success | Failure
            _ => {},
        }
    }
}

fn generate_wander_path(
    rng: &mut impl RngCore,
    ai_pos: IVec2,
    movement_type: u8,
    map_provider: &impl PathProvider,
) -> Option<Vec<IVec2>> {
    let wander_radius = WANDER_RANGE.sample(rng);
    let wander_circle = Circle::new(ai_pos, wander_radius);

    // Default to the circle center
    let destination = wander_circle.iter().choose(rng).unwrap_or_else(|| wander_circle.center());
    PathFinder::Astar.compute(ai_pos, destination, movement_type, true, map_provider)
}
