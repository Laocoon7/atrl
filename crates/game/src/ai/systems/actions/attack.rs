use big_brain::actions::ActionState;

use crate::prelude::*;

#[derive(Debug, Default, Component, Clone)]
// could be used for temporary storage for multi turn actions
pub struct AttackActor;

pub fn attack_player(
    mut commands: Commands,
    player_q: Query<(Entity, &Position), With<Player>>,
    mut target_q: Query<&mut TargetVisualizer>,
    mut action_q: Query<(&Actor, &mut ActionState), With<AttackActor>>,
    mut ai_q: Query<(&Position, &Name, &mut AIComponent), Without<Player>>,
) {
    use ActionState::*;

    let (player_entity, player_position) = match player_q.get_single() {
        Ok(p) => p,
        Err(err) => {
            error!("No player found: {}", err);
            return;
        },
    };

    action_q.iter_mut().for_each(|(Actor(actor), mut action_state)| {
        let Ok((ai_position, name, mut ai_component)) =
            ai_q.get_mut(*actor) else {
                info!("Actor must have required attack components");
                return
            };

        if ai_component.preferred_action.is_some() {
            // already attacking, quick return;
            return;
        }

        // let Some(map) = manager.get_current_map() else {
        //     info!("No map found");
        //     return
        // };

        match *action_state {
            // Success | Failure
            Success | Failure => {
                // Nothing to do here
                info!("{} attack state: {:?}", name, action_state);
                return;
            },
            ActionState::Cancelled => {
                info!("{} cancelled attack!", name);
                *action_state = Failure;
                ai_component.preferred_action = None;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }

                return;
            },

            // these final two fall through to logic
            ActionState::Init | ActionState::Requested => {
                info!("{} gonna start attacking!", name);
                *action_state = Executing;
                ai_component.preferred_action = Some(ActionType::Attack(player_entity));
            },
            ActionState::Executing => {},
        }

        let ai_pos = ai_position.gridpoint();
        let player_pos = player_position.gridpoint();

        if in_attack_range(ai_pos, player_pos) {
            ai_component.preferred_action = Some(ActionType::Wait);
            *action_state = ActionState::Success;
        } else {
            ai_component.preferred_action = Some(ActionType::Wait);
            *action_state = ActionState::Failure;
        }
    });
}
