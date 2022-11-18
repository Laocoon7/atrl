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
    _fail_behavior: ChaseActorFailureBehavior,
}

pub fn chase_actor(
    player_q: Query<(Entity, &Transform), With<Player>>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut ChaseActor)>,
) {
    println!("Chase actor action");
    use ActionState::*;

    let (_player_entity, player_pos) = player_q.single();
    for (Actor(_actor), mut action_state, mut chase) in action_q.iter_mut() {
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
                chase.last_seen_pt = Some(player_pos.get());
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {}
        }

        // Do chasing things :)
        *action_state = ActionState::Failure;
    }
}
