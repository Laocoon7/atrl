use crate::prelude::*;
use bevy::render::once_cell::sync::Lazy;
use rand::distributions::Uniform;
use std::{collections::VecDeque, ops::RangeInclusive};

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
    manager: Res<MapManager>,
    mut ctx: ResMut<GameContext>,
    mut spatial_q: Query<(&mut Transform, &Movement)>,
    mut action_q: Query<(&Name, &Actor, &mut BigBrainActionState, &mut Wander)>,
) {
    use BigBrainActionState::*;

    for (name, Actor(actor), mut action_state, mut wander) in action_q.iter_mut() {
        if let Ok((mut position, movement_component)) = spatial_q.get_mut(*actor) {
            match *action_state {
                Init => {
                    println!("Wander init");
                    continue;
                }
                Success => {
                    println!("Wander success");
                    continue;
                }
                Cancelled => {
                    println!("Wander cancelled");
                    *action_state = Failure;
                    continue;
                }
                Failure => {
                    println!("Wander failed");
                    continue;
                }

                // These final two fall through to logic
                Requested => {
                    println!("{} gonna start wandering!", name);
                    *action_state = Executing;
                }
                Executing => {}
            }

            if let Some(map) = manager.get_current_map() {
                let ai_pos = position.get();
                let rng = ctx.random.get_prng().as_rngcore();

                if wander.path.is_none() {
                    wander.path =
                        Some(generate_wander_path(rng, &ai_pos, movement_component.0, map));
                }

                let ai_path = wander.path.as_mut().unwrap();
                println!("Wander path: starting at: {}  path: {:?}", ai_pos, ai_path);
                if let Some(next_pt) = ai_path.get(1) {
                    println!("Wander path: next point: {}", next_pt);
                    if map.can_move_through(*next_pt, **movement_component) {
                        position.set_value(*next_pt);
                    } else {
                        *action_state = Failure;
                    }

                    ai_path.remove(1);
                } else {
                    // We have reached the end of our trail!
                    *action_state = Success;
                    info!("{} has reached the end of their wander path!", name);
                }
            } else {
                *action_state = Failure;
            }
        }
    }
}

fn generate_wander_path(
    rng: &mut impl RngCore,
    ai_pos: &IVec2,
    movement_type: u8,
    map_provider: &impl PathProvider,
) -> Vec<IVec2> {
    let wander_radius = WANDER_RANGE.sample(rng);
    let wander_circle = Circle::new(*ai_pos, wander_radius);
    // Default to the first point in the circle
    let destination = wander_circle.iter().choose(rng).unwrap_or_else(|| wander_circle.points()[0]);
    PathFinder::Astar.compute(*ai_pos, destination, movement_type, map_provider).unwrap()
}
