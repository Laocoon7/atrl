use std::collections::VecDeque;

use big_brain::actions::ActionState;

use crate::prelude::*;

#[derive(Debug, Default, Component, Clone)]
pub struct ChaseActor {
    generated_path: bool,
    last_seen_pt: Option<Position>,
}

pub fn chase_action(
    tilesets: Tilesets,
    mut commands: Commands,
    manager: Res<MapManager>,
    player_q: Query<&Position, With<Player>>,
    mut target_q: Query<&mut TargetVisualizer>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut ChaseActor)>,
    mut ai_q: Query<
        (
            &Position,
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

    let player_position = match player_q.get_single() {
        Ok(p) => p,
        Err(err) => {
            error!("No player found: {}", err);
            return;
        },
    };

    for (Actor(actor), mut action_state, mut chase) in action_q.iter_mut() {
        let Ok((ai_position, fov, movement,vision, name, mut ai_component)) =
            ai_q.get_mut(*actor) else {
                info!("Actor must have required components");
                continue;
            };

        // if ai_component.preferred_action.is_some() {
        //     // already chasing, quick return;
        //     continue;
        // }

        let Some(map) = manager.get_current_map() else {
            info!("No map found");
            continue;
        };

        let ai_pos = ai_position.gridpoint();
        let player_pos = player_position.gridpoint();

        match *action_state {
            // Success | Failure
            Success | Failure => {
                info!("{} chase state: {:?}", name, action_state);
                ai_component.preferred_action = None;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }

                continue;
            },
            Cancelled => {
                info!("{} cancelled chase!", name);
                *action_state = Failure;
                ai_component.preferred_action = None;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }

                continue;
            },
            Init | Requested => {
                info!("{} gonna start chasing!", name);
                *action_state = Executing;

                chase.generated_path = false;
                chase.last_seen_pt = Some(*player_position);
                ai_component.preferred_action = Some(ActionType::Movement(*player_position));
            },
            Executing => {},
        }

        info!("{} executing chase!", name);

        let position = if entity_in_fov(map, fov, vision, ai_pos, player_pos) {
            let player_pos = *player_position;
            if in_attack_range(ai_pos, player_pos.gridpoint()) {
                info!("{} in attack range!", name);
                *action_state = Success;
                continue;
            }

            chase.last_seen_pt = Some(player_pos);
            chase.generated_path = false;
            player_pos
        } else {
            let Some(last_seen) = chase.last_seen_pt else {
                        error!("Executing chase with no target.");
                        ai_component.preferred_action = Some(ActionType::Wait);
                        continue;
                    };

            // We reached the end of our chase path and we do not see the player :(
            if last_seen.gridpoint() == ai_pos {
                *action_state = Failure;
                continue;
            }

            // We have lost sight of the player and need a path to their last seen position.
            // Our pathfinder will only generate a valid path to the last seen location, this includes
            // partial path. We can expect the first element in the path to be a valid location
            // that is closest to the last_seen_pt.
            if !chase.generated_path {
                let xy = generate_last_seen_path(ai_pos, last_seen.gridpoint(), movement.0, map)
                    .first()
                    .unwrap_or(&last_seen.gridpoint().as_ivec2())
                    .as_uvec2();
                let last_seen_pt = Position::new(
                    WorldPosition::new(
                        last_seen.world_x(),
                        last_seen.world_y(),
                        last_seen.world_z(),
                    ),
                    LocalPosition::new(xy.x, xy.y, MapLayer::Actors as u32),
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
                ai_position.gridpoint(),
                position.gridpoint(),
                Color::RED,
            );
        }
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
