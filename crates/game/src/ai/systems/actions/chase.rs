use big_brain::actions::ActionState;

use crate::prelude::*;

#[derive(Debug, Default, Component, Clone)]
pub struct ChaseActor {
    last_seen_pt: Option<(IVec3, UVec2)>,
}

pub fn chase_action(
    tilesets: Tilesets,
    mut commands: Commands,
    mut manager: ResMut<MapManager>,
    mut target_q: Query<&mut TargetVisualizer>,
    player_q: Query<(Entity, &WorldPosition, &LocalPosition), With<Player>>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut ChaseActor, &ActionSpan)>,
    mut ai_q: Query<
        (
            &WorldPosition,
            &LocalPosition,
            &FieldOfView,
            &Vision,
            &Name,
            &mut AIComponent,
        ),
        Without<Player>,
    >,
) {
    use ActionState::*;

    for (Actor(actor), mut action_state, mut chase, span) in action_q.iter_mut() {
        let _guard = span.span().enter();

        let (_player_entity, player_world_position, player_local_position) = player_q.single();
        let Ok((_ai_world_position, ai_local_position, fov, vision, name, mut ai_component)) =
            ai_q.get_mut(*actor) else {
                error!("Actor must have required components");
                return
            };

        if ai_component.preferred_action.is_some() {
            // already chasing, quick return;
            commands.insert_resource(TurnState::Processing);
            return;
        }

        let ai_pos = ai_local_position.0;
        let player_pos = player_local_position.0;
        let Some(map) = manager.get_current_map_mut() else {
            error!("No map found");
            return
        };

        match *action_state {
            Requested => {
                let position = (player_world_position.0, player_local_position.0);
                chase.last_seen_pt = Some(position);
                ai_component.preferred_action = Some(ActionType::Movement(position));
                *action_state = Executing;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.update(
                        &mut commands,
                        &tilesets,
                        ai_local_position.0,
                        player_local_position.0,
                        Color::RED,
                    );
                }

                info!("{} gonna start chasing!", name);
            },
            Cancelled => {
                ai_component.preferred_action = None;
                info!("{} cancelled chase!", name);
                *action_state = Failure;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }
            },
            Executing => {
                info!("{} executing chase!", name);

                let position = if entity_in_fov(map, fov, vision, ai_pos, player_pos) {
                    let player_pos = (player_world_position.0, player_local_position.0);
                    chase.last_seen_pt = Some(player_pos);
                    player_pos
                } else {
                    let Some(last_seen) = chase.last_seen_pt else {
                        error!("Executing chase with no target.");
                        ai_component.preferred_action = Some(ActionType::Wait);
                        return;
                    };

                    last_seen
                };

                ai_component.preferred_action = Some(ActionType::Movement(position));

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.update(
                        &mut commands,
                        &tilesets,
                        ai_local_position.0,
                        player_local_position.0,
                        Color::RED,
                    );
                }
            },

            // Init | Success | Failure
            _ => {},
        }

        info!("Chase action output: {:?}\n", action_state);
    }
}
