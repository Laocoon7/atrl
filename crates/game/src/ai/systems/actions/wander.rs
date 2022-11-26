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
    mut spatial_q: Query<(&mut Transform, &Movement, &Name)>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut Wander, &ActionSpan)>,
) {
    use ActionState::*;

    for (Actor(actor), mut action_state, mut wander, span) in action_q.iter_mut() {
        let _guard = span.span().enter();

        let rng = ctx.random.get_prng().as_rngcore();
        let Some(map) = manager.get_current_map_mut() else {
            error!("No map found");
            return
        };

        let Ok((mut position, movement_component, name)) =
        spatial_q.get_mut(*actor,) else {
                error!("Actor must have spatial components");
                return
            };

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
                wander.path = Some(generate_wander_path(rng, ai_pos, movement_component.0, map));
            },
            Executing => {
                let wander_path = std::mem::take(&mut wander.path);
                let ai_path = wander_path.and_then(|p| {
                    if p.is_empty() || !map.can_place_actor(p[0], movement_component.0) {
                        None
                    } else {
                        Some(p)
                    }
                });

                let mut ai_path = ai_path.map_or_else(
                    || generate_wander_path(rng, ai_pos, movement_component.0, map),
                    |p| p,
                );

                ai_path.pop().map_or_else(
                    || {
                        // We have reached the end of our trail!
                        *action_state = Success;
                        info!("{} has reached the end of their wander path!", name);
                    },
                    |next_pt| {
                        if map.try_move_actor(ai_pos, next_pt, movement_component.0) {
                            position.set_value(next_pt);
                            if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                                if !ai_path.is_empty() {
                                    target_visualizer.update(
                                        &mut commands,
                                        &tilesets,
                                        next_pt,
                                        ai_path[0],
                                        Color::WHITE,
                                    );
                                } else {
                                    target_visualizer.update(
                                        &mut commands,
                                        &tilesets,
                                        next_pt,
                                        next_pt,
                                        Color::WHITE,
                                    );
                                }
                            }
                        } else {
                            info!(
                                "AI is blocked trying to move from {:?} to {:?}",
                                ai_pos, next_pt
                            );
                        }
                    },
                );
                wander.path = Some(ai_path);
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
) -> Vec<IVec2> {
    let wander_radius = WANDER_RANGE.sample(rng);
    let wander_circle = Circle::new(ai_pos, wander_radius);

    // Default to the circle center
    let destination = wander_circle.iter().choose(rng).unwrap_or_else(|| wander_circle.center());
    PathFinder::Astar
        .compute(ai_pos, destination, movement_type, false, map_provider)
        .unwrap_or_default()
}
