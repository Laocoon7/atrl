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
    map_q: Query<&GameMap>,
    mut action_q: Query<(
        &Name,
        &Actor,
        &mut ActionState,
        &Wander,
        &mut Transform,
        &WorldPosition,
        &Movement,
    )>,
) {
    use ActionState::*;

    for (
        name,
        Actor(_actor),
        mut action_state,
        _wander,
        mut position,
        world_pos,
        movement_component,
    ) in action_q.iter_mut()
    {
        match *action_state {
            Executing => {}
            Init => {
                println!("Wander init");
                continue;
            }
            Requested => {
                println!("{} gonna start wandering!", name);
                *action_state = ActionState::Executing;
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
        }

        if let Some(map) = map_q.iter().find(|map| map.world_position == *world_pos) {
            let rng = &mut ctx.get_random().prng;
            let random_direction = rng.sample::<GridDirection>();

            let position_vec = position.get();
            let destination = position_vec + random_direction.coord().as_vec2();

            if map.can_move_through(destination, movement_component) {
                info!("{} moved to {:?}", name, destination);
                position.set_value(destination);
            } else {
                *action_state = ActionState::Failure;
            }
        } else {
            *action_state = ActionState::Failure;
        }
    }
}
