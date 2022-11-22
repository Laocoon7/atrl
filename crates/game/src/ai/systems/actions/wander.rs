use crate::prelude::*;
use bevy::render::once_cell::sync::Lazy;
use rand::distributions::Uniform;
use std::ops::RangeInclusive;

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
    // What to do if entity has no available places to move
    fail_behavior: WanderFailureBehavior,
}

pub fn wander_action(
    manager: Res<MapManager>,
    mut ctx: ResMut<GameContext>,
    mut action_q: Query<(&Name, &Actor, &mut BigBrainActionState, &Wander)>,
    mut spatial_q: Query<(&mut Transform, &Movement)>,
) {
    use BigBrainActionState::*;

    for (name, Actor(actor), mut action_state, _wander) in action_q.iter_mut() {
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
                let position_vec = position.get();

                let mut rng = ctx.random.get_prng().as_rngcore();
                let wander_radius = WANDER_RANGE.sample(&mut rng);
                let wander_circle = Circle::new(position_vec, wander_radius);

                if let Some(destination) = wander_circle.iter().choose(&mut rng) {
                    if map.can_move_through(destination, movement_component) {
                        position.set_value(destination);
                    } else {
                        *action_state = Failure;
                    }
                } else {
                    error!("No wander destination found! Moving to failed state");
                    *action_state = Failure;
                }
            } else {
                *action_state = Failure;
            }
        } else {
            *action_state = Failure;
        }
    }
}
