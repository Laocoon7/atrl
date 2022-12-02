use std::{collections::VecDeque, ops::RangeInclusive};

use bevy::render::once_cell::sync::Lazy;
use big_brain::actions::ActionState;
use rand::distributions::Uniform;

use crate::prelude::*;

static WANDER_RANGE: Lazy<Uniform<u32>> = Lazy::new(|| Uniform::new_inclusive(3, 10));

// could be used for temporary storage for multi turn actions
#[derive(Debug, Reflect, Component, Clone, Eq, PartialEq)]
#[reflect(Component)]
pub struct Wander {
    my_previous_location: Position,
    destination: Option<Position>,
}

impl Default for Wander {
    fn default() -> Self {
        Self {
            my_previous_location: Position::new(
                WorldPosition::new(0, 0, 0),
                LocalPosition::new(0, 0, MapLayer::Actors as u32),
            ),
            destination: None,
        }
    }
}

pub fn wander_action(
    tilesets: Tilesets,
    mut commands: Commands,
    manager: Res<MapManager>,
    mut ctx: ResMut<GameContext>,
    mut target_q: Query<&mut TargetVisualizer>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut Wander)>,
    mut spatial_q: Query<(&Position, &Movement, &Name, &mut AIComponent), Without<Player>>,
) {
    use ActionState::*;

    action_q.iter_mut().for_each(|(Actor(actor), mut action_state, mut wander)| {
        let rng = ctx.random.get_prng().as_rngcore();
        let Some(map) = manager.get_current_map() else {
            info!("No map found");
            return
        };

        let Ok((ai_position, movement,name, mut ai_component)) =
        spatial_q.get_mut(*actor) else {
            info!("Actor must have spatial components");
                return
            };

        if ai_component.preferred_action.is_some() {
            // already wandering, quick return;
            return;
        }

        match *action_state {
            // Success | Failure
            Success | Failure => {
                // Nothing to do here
                info!("{} wander state: {:?}", name, action_state);
                return;
            },
            Cancelled => {
                info!("{} cancelled wander", name);
                ai_component.preferred_action = None;
                *action_state = Failure;

                // commands.entity(*actor).remove::<Wander>().insert(ChaseActor::default());

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }

                return;
            },

            // These two states will fall through to execution
            Init | Requested => {
                info!("{} gonna start wandering!", name);
                *action_state = Executing;
            },
            Executing => {},
        }

        info!("{} executing wander!", name);

        let destination = match std::mem::take(&mut wander.destination) {
            Some(destination) => {
                if Line::new(ai_position.xy(), destination.xy()).get_count() <= 1 {
                    let xy = generate_wander_path(rng, map, ai_position.xy(), movement.0).as_uvec2();
                    Position::new(
                        WorldPosition::new(
                            ai_position.world_x(),
                            ai_position.world_y(),
                            ai_position.world_z(),
                        ),
                        LocalPosition::new(xy.x, xy.y, MapLayer::Actors as u32),
                    )
                } else {
                    destination
                }
            },
            None => {
                let xy = generate_wander_path(rng, map, ai_position.xy(), movement.0).as_uvec2();
                Position::new(
                    WorldPosition::new(
                        ai_position.world_x(),
                        ai_position.world_y(),
                        ai_position.world_z(),
                    ),
                    LocalPosition::new(xy.x, xy.y, MapLayer::Actors as u32),
                )
            },
        };

        wander.destination = Some(destination);
        wander.my_previous_location = *ai_position;
        ai_component.preferred_action = Some(ActionType::Movement(destination));

        if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
            target_visualizer.update(
                &mut commands,
                &tilesets,
                ai_position.xy(),
                destination.xy(),
                Color::WHITE,
            );
        }
    });
}

fn generate_wander_path(rng: &mut impl RngCore, map: &Map, ai_pos: UVec2, movement_type: u8) -> IVec2 {
    let wander_radius = WANDER_RANGE.sample(rng);
    let wander_circle = Circle::new(ai_pos, wander_radius);

    loop {
        // Default to the circle center
        let new_destination = wander_circle.iter().choose(rng).unwrap_or_else(|| wander_circle.center());
        if map.can_place_actor(new_destination, movement_type) {
            return new_destination;
        }
    }
}
