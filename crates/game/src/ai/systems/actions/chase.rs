use std::collections::VecDeque;

use big_brain::actions::ActionState;

use crate::prelude::*;

#[derive(Debug, Default, Component, Clone)]
pub struct ChaseActor {
    generated_path: bool,
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
            &Movement,
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
        let Ok((_ai_world_position, ai_local_position, fov, movement,vision, name, mut ai_component)) =
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
            // Init | Success | Failure
            Init | Success | Failure => {
                // Nothing to do here
                continue;
            },
            Cancelled => {
                ai_component.preferred_action = None;
                info!("{} cancelled chase!", name);
                *action_state = Failure;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }
            },
            Requested => {
                info!("{} gonna start chasing!", name);
                *action_state = Executing;

                let position = (player_world_position.0, player_local_position.0);
                ai_component.preferred_action = Some(ActionType::Movement(position));

                chase.generated_path = false;
                chase.last_seen_pt = Some(position);
            },
            Executing => {},
        }

        info!("{} executing chase!", name);

        let position = if entity_in_fov(map, fov, vision, ai_pos, player_pos) {
            let player_pos = (player_world_position.0, player_local_position.0);
            chase.last_seen_pt = Some(player_pos);
            chase.generated_path = false;
            player_pos
        } else {
            let Some(last_seen) = chase.last_seen_pt else {
                        error!("Executing chase with no target.");
                        ai_component.preferred_action = Some(ActionType::Wait);
                        return;
                    };

            // We reached the end of our chase path and we do not see the player :(
            if last_seen.1 == ai_pos {
                // Failed or Success? Either works since we dont have anything happen in success or failure
                *action_state = Failure;
                continue;
            }

            // We have lost sight of the player and need a path to their last seen position.
            // Our pathfinder will only generate a valid path to the last seen location, this includes
            // partial path We can expect the first element in the path to be a valid location
            // that is closest to the last_seen_pt.
            if !chase.generated_path {
                let last_seen_pt = (
                    last_seen.0,
                    generate_last_seen_path(ai_pos, last_seen.1, movement.0, map)
                        .first()
                        .unwrap_or(&last_seen.1.as_ivec2())
                        .as_uvec2(),
                );

                chase.generated_path = true;
                chase.last_seen_pt = Some(last_seen_pt);
                last_seen_pt
            } else {
                last_seen
            }
        };

        ai_component.preferred_action = Some(ActionType::Movement(position));

        if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
            target_visualizer.update(
                &mut commands,
                &tilesets,
                ai_local_position.0,
                position.1,
                Color::RED,
            );
        }

        info!("Chase action output: {:?}\n", action_state);
    }
}

fn generate_last_seen_path(
    ai_pos: UVec2,
    target_pos: UVec2,
    movement_type: u8,
    map_provider: &impl PathProvider,
) -> Vec<IVec2> {
    PathFinder::Astar
        .compute(
            ai_pos.as_ivec2(),
            target_pos.as_ivec2(),
            movement_type,
            true,
            map_provider,
        )
        .unwrap_or_default()
}
