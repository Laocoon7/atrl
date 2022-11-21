use crate::prelude::*;
use big_brain::actions::ActionState;

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
    ctx: Res<GameContext>,
    manager: Res<MapManager>,
    mut action_q: Query<(&Name, &Actor, &mut ActionState, &Wander)>,
    mut spatial_q: Query<(&mut Transform, &Movement)>,
) {
    use ActionState::*;

    for (name, Actor(actor), mut action_state, _wander) in action_q.iter_mut() {
        if let Ok((mut position, movement_component)) = spatial_q.get_mut(*actor) {
            match *action_state {
                Init => {
                    println!("Wander init");
                    continue;
                }
                ActionState::Success => {
                    println!("Wander success");
                    continue;
                }
                Cancelled => {
                    println!("Wander cancelled");
                    *action_state = ActionState::Failure;
                    continue;
                }
                Failure => {
                    println!("Wander failed");
                    continue;
                }

                // These final two fall through to logic
                Requested => {
                    println!("{} gonna start wandering!", name);
                    *action_state = ActionState::Executing;
                }
                Executing => {}
            }

            if let Some(map) = manager.get_current_map() {
                let rng = &mut ctx.get_random().prng;
                let random_direction = rng.sample::<GridDirection>();

                let position_vec = position.get();
                let destination = position_vec + random_direction.coord().as_vec2();

                if map.can_move_through(destination, movement_component) {
                    position.set_value(destination);
                } else {
                    *action_state = ActionState::Failure;
                }
            } else {
                *action_state = ActionState::Failure;
            }
        } else {
            *action_state = ActionState::Failure;
        }
    }
}
