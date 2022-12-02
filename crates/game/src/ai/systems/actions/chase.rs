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
    mut manager: ResMut<MapManager>,
    mut target_q: Query<&mut TargetVisualizer>,
    player_q: Query<(Entity, &Position), With<Player>>,
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

    action_q.iter_mut().for_each(|(Actor(actor), mut action_state, mut chase)| {
        let (_player_entity, player_position) = player_q.single();
        let Ok((ai_position, fov, movement,vision, name, mut ai_component)) =
            ai_q.get_mut(*actor) else {
                info!("Actor must have required components");
                return
            };

        if ai_component.preferred_action.is_some() {
            // already chasing, quick return;
            return;
        }

        let ai_pos = ai_position.xy();
        let player_pos = player_position.xy();
        let Some(map) = manager.get_current_map_mut() else {
            info!("No map found");
            return
        };

        match *action_state {
            // Success | Failure
            Success | Failure => {
                // Nothing to do here
                info!("{} chase state: {:?}", name, action_state);
                return;
            },
            Cancelled => {
                ai_component.preferred_action = None;
                info!("{} cancelled chase!", name);
                *action_state = Failure;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }

                return;
            },
            Init | Requested => {
                info!("{} gonna start chasing!", name);
                *action_state = Executing;

                ai_component.preferred_action = Some(ActionType::Movement(*player_position));

                chase.generated_path = false;
                chase.last_seen_pt = Some(*player_position);
            },
            Executing => {},
        }

        info!("{} executing chase!", name);

        let position = if entity_in_fov(map, fov, vision, ai_pos, player_pos) {
            let player_pos = *player_position;
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
            if last_seen.xy() == ai_pos {
                // Failed or Success? Either works since we dont have anything happen in success or failure
                *action_state = Failure;
                return;
            }

            // We have lost sight of the player and need a path to their last seen position.
            // Our pathfinder will only generate a valid path to the last seen location, this includes
            // partial path We can expect the first element in the path to be a valid location
            // that is closest to the last_seen_pt.
            if !chase.generated_path {
                let xy = generate_last_seen_path(ai_pos, last_seen.xy(), movement.0, map)
                    .first()
                    .unwrap_or(&last_seen.xy().as_ivec2())
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
                ai_position.xy(),
                position.xy(),
                Color::RED,
            );
        }
    });
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
